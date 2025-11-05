# "slim" means it has removed unnecessary tools to keep it small.
FROM debian:bullseye-slim

# We chain commands with '&&' to keep the Docker "layer" clean.
# We install:
    # - ruby-full: Needed because 'zsteg' is a Ruby gem.
    # - binwalk, libimage-exiftool-perl (exiftool), binutils (strings): Core CTF tools.
RUN apt-get update && apt-get install -y \
    ruby-full \
    binwalk \
    libimage-exiftool-perl \
    binutils \
    && gem install zsteg \
    && rm -rf /var/lib/apt/lists/*

# 3. THE WORKSPACE: Set the default directory for the user
WORKDIR /challenges

# This copies everything from your current folder (.) to the container's folder (.)
COPY flag.txt .

# /bin/bash lets you interact with it like a normal terminal.
CMD ["/bin/bash"]