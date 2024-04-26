package tree_sitter_resources_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-resources"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_resources.Language())
	if language == nil {
		t.Errorf("Error loading Resources grammar")
	}
}
