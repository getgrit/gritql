import React from 'react';

class SampleComponent extends React.Component {
  onClick = () => {
    this.clicks = this.clicks + 1;
  };

  @observable
  private clicks = this.props.initialCount;

  @computed
  private get isEven() {
    return this.clicks % 2 === 0;
  }


  render() {
    return (
        <>
            <p>Clicks: {this.clicks}</p>
            <p>Is even: {this.isEven}</p>
            <a onClick={this.onClick}>click</a>
        </>
    );
  }
}
