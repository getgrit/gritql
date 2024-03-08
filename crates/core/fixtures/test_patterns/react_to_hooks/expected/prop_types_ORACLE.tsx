import React from "react";

interface Props {
  name: string;
}

const SampleComponent = (props: Props) => {
  return (
    <>
      <p>Hello {props.name}</p>
    </>
  );
};
