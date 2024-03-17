import React, { useState } from 'react';

interface Person {
  name: string;
}

const ObservedComponent = (inputProps) => {
    const [me, setMe] = useState<Person>({
    name: "John",
  });

    const props = { 
    king: "viking",
    ...inputProps,
  };

    return (
      <>
        <p>This is {me.name}, {props.king}</p>
      </>
    ); 
};

