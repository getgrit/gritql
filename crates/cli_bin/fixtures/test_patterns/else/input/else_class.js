class Other {
  async render() {
    const { href } = this.props;

    return <a href={href}>Link Text</a>;
  }
}
