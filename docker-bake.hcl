target "default" {
  context    = "."
  dockerfile = "ci/build.Dockerfile"
  output     = ["type=local,dest=ci/build"]
}

target "demo" {
  context    = "."
  contexts   = {
    builder = "target:default"
  }
  dockerfile = "ci/demo.Dockerfile"
  args = {
    BUILDKIT_SANDBOX_HOSTNAME = "my-desktop"
  }
  output = ["type=local,dest=ci/"]
}

