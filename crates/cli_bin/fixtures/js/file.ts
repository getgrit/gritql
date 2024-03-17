import { RichFile } from '@getgrit/universal';
import { createContext, useContext } from 'react';

export interface PatternTester {
  analyzeFile: (file: RichFile, pattern?: string) => void;
  analyzeTestFiles: (pattern?: string) => void;
}
