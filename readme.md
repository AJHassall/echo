# Echo: Real-time Desktop Audio Captions (Early Development)


**This project is a very early experiment and is unstable. Use with caution.**

Echo captures your computer's audio and creates real-time captions using a local AI model (Whisper).

## Build Steps

To build and run the application, follow these steps:

1.  **Clone the Repository:**

    ```bash
    git clone <repository_url>
    ```

2.  **Download Whisper Model:**

    ```bash
    cd whisper_models
    ./dl.sh tiny.en
    cd ..
    ```
    * This step downloads the "tiny.en" Whisper model, which is required for the application.

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

**Current Status:**

* Captures desktop audio.
* Transcribes audio using a local Whisper model.
* Displays transcriptions in the console.

**Key Technologies:**

* Rust (for performance)
* Neon (Rust to JavaScript bridge)
* Electron (cross-platform desktop app)
* Whisper (local audio transcription)

**Planned Features:**

* Real-time caption overlay on the desktop.
* Cross-platform support (Windows, macOS, Linux).
* Improved stability and error handling.
* Better audio processing (noise reduction, etc.).
* User settings (audio device, language, caption style, model selection).
* Code improvements (remove static variables).
* Improved user interface.
* Speaker identification.
* Saving transcriptions.


