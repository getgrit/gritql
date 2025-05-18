interface Match {
  type: 'match';
  variables: Record<string, any>;
}

interface State {
  state: 'loading' | 'loaded' | 'error';
  result?: Match;
}

export const extractMetavariables = (state: State) => {
  if (state.state !== 'loaded' || !state.result || state.result.type !== 'match') {
    return {
      metaVariables: [],
      oldVariables: [],
      newVariables: [],
    };
  }

  const variables = Object.entries(state.result.variables).map(([name, value]) => ({
    name,
    value,
  }));

  return {
    metaVariables: variables,
    oldVariables: variables,
    newVariables: variables,
  };
}; 