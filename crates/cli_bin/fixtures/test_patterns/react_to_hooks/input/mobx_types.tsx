import React from "react";

interface Person {
  name: string;
}

class ObservedComponent extends React.Component {
  static defaultProps = { king: "viking" };

  @observable
  private me: Person = {
    name: "John",
  };

  render() {
    return (
      <>
        <p>This is {this.me.name}, {this.props.king}</p>
      </>
    );
  }
}
