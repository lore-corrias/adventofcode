FROM debian:stable

# Install devtools
RUN apt-get update -y && \
  apt-get install -y git zsh nodejs fd-find ripgrep gpg curl wget tmux zoxide wl-clipboard npm

# Install fzf
RUN git clone --depth 1 https://github.com/junegunn/fzf.git ~/.fzf && \
  ~/.fzf/install

# Install neovim from source
RUN curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim.appimage && \
  chmod u+x nvim.appimage && \
  ./nvim.appimage --appimage-extract && \
  ./squashfs-root/AppRun --version && \
  ln -s /squashfs-root/AppRun /usr/bin/nvim

# Install eza
RUN mkdir -p /etc/apt/keyrings && \
  wget -qO- https://raw.githubusercontent.com/eza-community/eza/main/deb.asc | gpg --dearmor -o /etc/apt/keyrings/gierens.gpg && \
  echo "deb [signed-by=/etc/apt/keyrings/gierens.gpg] http://deb.gierens.de stable main" | tee /etc/apt/sources.list.d/gierens.list && \
  chmod 644 /etc/apt/keyrings/gierens.gpg /etc/apt/sources.list.d/gierens.list && \
  apt-get update && \
  apt-get install -y eza

ARG USERNAME=devcontainer
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Create the user
RUN groupadd --gid $USER_GID $USERNAME \
  && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
  #
  # [Optional] Add sudo support. Omit if you don't need to install software after connecting.
  && apt-get update \
  && apt-get install -y sudo \
  && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
  && chmod 0440 /etc/sudoers.d/$USERNAME

# ********************************************************
# * Anything else you want to do like clean up goes here *
# ********************************************************

# [Optional] Set the default user. Omit if you want to keep the default as root.
USER $USERNAME
