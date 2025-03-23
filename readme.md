## Build Steps

To build and run the application, follow these steps:

1.  **Clone the Repository:**

    ```bash
    git clone git@github.com:AJHassall/echo.git
    ```

2.  **Download Whisper Model (from whisper.cpp):**

    ```bash
    cd whisper_models
    ./dl.sh tiny.en
    cd ..
    ```
    * This step downloads the "tiny.en" Whisper model, which is required for the application.
    * **Note:** The Whisper models are utilized through the [whisper.cpp](https://github.com/ggerganov/whisper.cpp) project, which provides a C++ implementation of the Whisper speech recognition model. This application leverages the whisper.cpp project for efficient model execution.

3.  **Install Dependencies and Build:**

    ```bash
    yarn install && yarn build:module && yarn start
    ```
    * `yarn install` installs the project's JavaScript dependencies.
    * `yarn build:module` builds the native Rust module.
    * `yarn start` starts the Electron application.

**Dependencies:**

Before building, ensure that the following dependencies are installed on your system:

* **Rust Compiler:** The Rust toolchain is required to compile the native Rust module. Install it from [rustup.rs](https://rustup.rs/).
* **OpenSSL Libraries:** The OpenSSL development libraries are required. Install them using your system's package manager (e.g., `sudo apt-get install libssl-dev` on Debian/Ubuntu, `brew install openssl` on macOS).
* **libclang-dev:** The libclang development libraries are required. Install them using your system's package manager (e.g., `sudo apt-get install libclang-dev` on Debian/Ubuntu).

**Credits:**

* This application utilizes the [whisper.cpp](https://github.com/ggerganov/whisper.cpp) project for efficient Whisper model execution.
* The Whisper models themselves are derived from the original OpenAI Whisper project.
