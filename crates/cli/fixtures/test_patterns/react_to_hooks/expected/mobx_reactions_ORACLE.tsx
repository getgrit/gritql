import React, { useCallback, useState } from 'react';

const SampleComponent = props => {
  const [clicks, setClicks] = useState(props.initialCount);

  const onClickHandler = useCallback(() => {
    setClicks(clicks + 1);
  }, []);

  useEffect(() => {
    console.log("clicks", clicks);
  }, [clicks]);

  useEffect(() => {
    console.log("second click handler");
  }, [props]);

  return <>
      <p>Clicks: {clicks}</p>
      <a onClick={onClickHandler}>click</a>
  </>;
};