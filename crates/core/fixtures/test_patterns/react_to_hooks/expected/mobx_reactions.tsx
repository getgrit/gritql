import React, { useState, useCallback, useEffect } from 'react';

const SampleComponent = (props) => {
    const [clicks, setClicks] = useState(props.initialCount);

    const onClickHandler = useCallback(() => {
    setClicks(clicks + 1);
  }, [clicks]);
    useEffect(() => {
     console.log("clicks", clicks);
   }, [clicks]);
    useEffect(() => {
     console.log("second click handler");
   }, []);

    return (
        <>
            <p>Clicks: {clicks}</p>
            <a onClick={onClickHandler}>click</a>
        </>
    ); 
};


