import React from "react";

class ObservedComponent extends React.Component {
  @observable
  private name?: string;

  @observable
  private age = 21;

  render() {
    const { name } = this;

    return (
      <>
        <p>Hello {name}, you are {this.age}</p>
      </>
    );
  }
}