// This is a smaller example
function() { 
  console.log("thing");
  const foo = () => {
    console.log("bar");
    const bar = () => {
      console.log("baz");
    }
  };
  // Not this
  // Delay a bit
  // Wow
  handle(() => { console.log("foo"); });
}