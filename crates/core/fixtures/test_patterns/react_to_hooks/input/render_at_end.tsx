import { Component } from "react";

class Link extends Component {
  render() {
    const { href } = this.props;

    return <a href={href}>Link Text</a>;
  }

  componentWillUnmount() {
    console.log("unmounted");
  }
}

