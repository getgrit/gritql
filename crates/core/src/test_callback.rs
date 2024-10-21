use grit_pattern_matcher::constants::DEFAULT_FILE_NAME;
use grit_pattern_matcher::pattern::ResolvedPattern;
use marzano_language::{grit_parser::MarzanoGritParser, target_language::TargetLanguage};
use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, AtomicUsize},
        Arc,
    },
};

use crate::{
    pattern_compiler::{CompilationResult, CompiledPatternBuilder},
    test_utils::{run_on_test_files, SyntheticFile},
};

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

#[test]
fn test_callback() {
    let src = r#"language js `console.log($_)`"#;
    let mut parser = MarzanoGritParser::new().unwrap();
    let src_tree = parser
        .parse_file(src, Some(Path::new(DEFAULT_FILE_NAME)))
        .unwrap();
    let lang = TargetLanguage::from_tree(&src_tree).unwrap();

    let callback_called = Arc::new(AtomicBool::new(false));
    let callback_called_clone = Arc::clone(&callback_called);

    assert!(!callback_called.load(std::sync::atomic::Ordering::SeqCst));

    let mut builder = CompiledPatternBuilder::start_empty(src, lang).unwrap();
    builder = builder.matches_callback(Box::new(move |binding, context, state, _logs| {
        let text = binding
            .text(&state.files, context.language)
            .unwrap()
            .to_string();
        assert_eq!(text, "console.log(\"hello\")");
        callback_called_clone.store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(true)
    }));
    let CompilationResult { problem, .. } = builder.compile(None, None, true).unwrap();

    let test_files = vec![SyntheticFile::new(
        "file.js".to_owned(),
        r#"function myLogger() {
            console.log("hello");
        }"#
        .to_owned(),
        true,
    )];
    let _results = run_on_test_files(&problem, &test_files);
    assert!(callback_called.load(std::sync::atomic::Ordering::SeqCst));
}

#[test]
fn test_rayon_parallelism() {
    let pool = rayon::ThreadPoolBuilder::new().build().unwrap();
    let num_threads = pool.current_num_threads();
    println!("Number of threads in the rayon pool: {}", num_threads);

    let tasks: Vec<_> = (0..num_threads * 100).collect();

    let last_task = Arc::new(AtomicUsize::new(0));

    fn run_task(task: usize, last_task: Arc<AtomicUsize>) {
        if task == 4 {
            std::thread::sleep(std::time::Duration::from_millis(3000));
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
        // println!("task completed: {}", task);
        last_task.store(task, std::sync::atomic::Ordering::SeqCst);
    }

    // Try a parallel iterator
    let start_par_iter = std::time::Instant::now();
    tasks.clone().into_par_iter().for_each(|task| {
        run_task(task, last_task.clone());
    });

    let duration_par_iter = start_par_iter.elapsed();
    println!("duration_par_iter: {:?}", duration_par_iter);
    println!(
        "last_task_par_iter: {}",
        last_task.load(std::sync::atomic::Ordering::SeqCst)
    );

    // Note how the iterator version is *slower* because it doesn't know that run_task varies so much in runtime,
    // so fast tasks end up queued behind slow task

    // Also try spawning threads manually
    let start_threads = std::time::Instant::now();
    rayon::scope(|s| {
        for task in tasks.clone() {
            let last_task_clone = Arc::clone(&last_task);
            s.spawn(move |_s| {
                run_task(task, last_task_clone);
            });
        }
    });

    let duration_threads = start_threads.elapsed();
    println!("duration_threads: {:?}", duration_threads);
    println!(
        "last_task_threads: {}",
        last_task.load(std::sync::atomic::Ordering::SeqCst)
    );

    // The thread version is fast, because once one thread is blocked on the slow task no more work is queued for it
    // The other threads finish the rest of the tasks before that thread wakes up
    assert_eq!(last_task.load(std::sync::atomic::Ordering::SeqCst), 4);
    // We should take under 3.1 seconds
    assert!(duration_threads < std::time::Duration::from_millis(3100));
}
