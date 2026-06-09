# Stage 1: Build the environment and run the demo
FROM debian:bookworm-slim AS demo-runner

# Install curl, ca-certificates, and less
RUN apt-get update && apt-get install -y curl ca-certificates less && rm -rf /var/lib/apt/lists/*

# Create user john and set up the installation directory
RUN useradd -m -d /home/john -s /bin/bash john
RUN mkdir -p /home/john/bin && chown -R john:john /home/john

# Set environment variables for evp installation
ENV EVP_VERSION=v0.10.0
ENV EVP_INSTALL_DIR=/home/john/bin
ENV PATH="/home/john/bin:${PATH}"

# Install evp
# RUN sh -c '/usr/bin/curl -sSfL https://raw.githubusercontent.com/HalFrgrd/evp/master/install.sh | sh'
COPY evp /home/john/bin/

# Copy the local univiz binary
COPY --from=builder /univiz /home/john/bin/univiz
RUN chmod +x /home/john/bin/univiz

# Set up the working directory
WORKDIR /app
RUN chown john:john /app

# Switch to john user
USER john

# Copy the demo.tape file
COPY --chown=john:john ci/demo.tape /app/demo.tape

# Run evp to generate the gif/svg
WORKDIR /home/john
RUN $EVP_INSTALL_DIR/evp /app/demo.tape

# Stage 2: Exporter stage to extract the generated gif
FROM scratch AS exporter
COPY --from=demo-runner /home/john/*.gif /home/john/*.svg /