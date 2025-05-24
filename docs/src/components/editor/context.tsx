import { createContext, useContext, useState, PropsWithChildren } from 'react';

export interface EditorState {
	pattern: string;
	setPattern: (newPattern: string) => void;
	input: string;
	setInput: (newInput: string) => void;
	path: string | undefined;
	setPath: (newPath: string) => void;
}

export const StandaloneEditorContext = createContext<EditorState>({
	pattern: "",
	setPattern: () => { },
	input: "",
	setInput: () => { },
	path: undefined,
	setPath: () => { },
});

export const StandaloneEditorProvider = ({ children }: PropsWithChildren<{}>) => {
	const [pattern, setPattern] = useState('');
	const [input, setInput] = useState('');
	const [path, setPath] = useState<string | undefined>(undefined);

	const value = {
		pattern,
		setPattern,
		input,
		setInput,
		path,
		setPath,
	};

	return (
		<StandaloneEditorContext.Provider value={value}>{children}</StandaloneEditorContext.Provider>
	);
};

export const useStandaloneEditor = () => {
	const context = useContext(StandaloneEditorContext);
	if (context === undefined) {
		throw new Error('useStandaloneEditor must be used within a StandaloneEditorProvider');
	}
	return context;
};
