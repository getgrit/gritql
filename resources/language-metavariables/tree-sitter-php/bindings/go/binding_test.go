package tree_sitter_php_base_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-php_base"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_php_base.Language())
	if language == nil {
		t.Errorf("Error loading PhpBase grammar")
	}
}
