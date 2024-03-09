import React, { useState, useCallback } from 'react';

const SampleComponent = (props) => {
    const [clicks, setClicks] = useState(props.initialCount);

    const onClickHandler = useCallback(() => {
    setClicks(clicks + 1);
  }, [clicks]);
    const isEven = useMemo(() => {
    return clicks % 2 === 0;
  }, [clicks]);

    return (
        <>
            <p>Clicks: {clicks}</p>
            <p>Is even: {isEven}</p>
            <a onClick={onClickHandler}>click</a>
        </>
    ); 
};


