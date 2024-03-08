import React from "react";

interface Props {
  name: string;
}

class SampleComponent extends React.Component<Props> {
  render() {
    return (
      <>
        <p>Hello {this.props.name}</p>
      </>
    );
  }
}
