target "default" {
  context    = "."
  dockerfile = "demo/demo.Dockerfile"
  args = {
    BUILDKIT_SANDBOX_HOSTNAME = "my-desktop"
  }
  output = ["type=local,dest=demo"]
}
