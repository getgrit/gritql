const Schema1 = v.string([v.email("Email required")]);
const Schema2 = v.string([v.email(), v.endsWith("@example.com")]);
const Schema3 = v.string([
  v.email(),
  v.endsWith("@example.com"),
  v.maxLength(30),
]);
