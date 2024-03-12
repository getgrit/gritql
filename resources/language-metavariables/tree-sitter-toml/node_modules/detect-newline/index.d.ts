declare const detectNewline: {
	/**
	Detect the dominant newline character of a string.

	@returns The detected newline or `undefined` when no newline character is found.

	@example
	```
	import detectNewline = require('detect-newline');

	detectNewline('foo\nbar\nbaz\r\n');
	//=> '\n'
	```
	*/
	(string: string): '\r\n' | '\n' | undefined;

	/**
	Detect the dominant newline character of a string.

	@returns Returns detected newline or `\n` when no newline character is found.
	*/
	graceful(string: string): '\r\n' | '\n';
};

export = detectNewline;
