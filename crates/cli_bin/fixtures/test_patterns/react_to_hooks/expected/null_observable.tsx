import React, { useState } from "react";

const ObservedComponent = () => {
  const [name, setName] = useState<string>(undefined);
  const [age, setAge] = useState(21);

  return (
    <>
      <p>
        Hello {name}, you are {age}
      </p>
    </>
  );
};
