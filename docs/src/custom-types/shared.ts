import { ReactNode } from 'react';

/*
 *
 * Types: Generic
 * Export any generic types you need within the project from here.
 *
 */
export type WithChildren<T = {}> = T & { children?: ReactNode };
