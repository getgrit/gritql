package tree_sitter_java_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-java"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_java.Language())
	if language == nil {
		t.Errorf("Error loading Java grammar")
	}
}
