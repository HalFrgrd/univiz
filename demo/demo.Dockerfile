# Stage 1: Build the environment and run the demo
FROM debian:bookworm-slim AS builder

# Install curl and ca-certificates to download evp
RUN apt-get update && apt-get install -y curl ca-certificates && rm -rf /var/lib/apt/lists/*

# Create user john and set up the installation directory
RUN useradd -m -d /home/john -s /bin/bash john
RUN mkdir -p /home/john/bin && chown -R john:john /home/john

# Set environment variables for evp installation
ENV EVP_VERSION=v0.10.0
ENV EVP_INSTALL_DIR=/home/john/bin
ENV PATH="/home/john/bin:${PATH}"

# Install evp
RUN sh -c '/usr/bin/curl -sSfL https://raw.githubusercontent.com/HalFrgrd/evp/master/install.sh | sh'

# Copy the local univiz binary
COPY target/debug/univiz /home/john/bin/univiz
RUN chmod +x /home/john/bin/univiz

# Set up the working directory
WORKDIR /app
RUN chown john:john /app

# Switch to john user
USER john

# Copy the demo.tape file
COPY --chown=john:john demo/demo.tape /app/demo.tape

# Run evp to generate the gif/svg
RUN evp demo.tape

# Default command if run interactively
CMD ["evp", "demo.tape"]

# Stage 2: Exporter stage to extract the generated gif
FROM scratch AS exporter
COPY --from=builder /app/univiz-demo.gif /univiz-demo.gif
COPY --from=builder /app/univiz-demo.svg /univiz-demo.svg
