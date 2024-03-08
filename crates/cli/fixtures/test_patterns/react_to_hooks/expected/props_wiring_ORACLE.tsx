import React, { useState } from "react";

interface Person {
  name: string;
}

const ObservedComponent = (inputProps: any) => {
  const props = {
    king: "viking",
    ...inputProps,
  };

  const [me, setMe] = useState<Person>({
    name: "John",
  });

  return (
    <>
      <p>
        This is {me.name}, {props.king}
      </p>
    </>
  );
};