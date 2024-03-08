import { Component } from 'react';

class SampleComponent extends Component {

  private viewState = new ViewState();

  render() {
    return (
        <p>This component has a <span onClick={this.viewState.click}>ViewState</span></p>
    );
  }
}
