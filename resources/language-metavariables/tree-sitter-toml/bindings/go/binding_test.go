package tree_sitter_toml_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-toml"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_toml.Language())
	if language == nil {
		t.Errorf("Error loading Toml grammar")
	}
}
