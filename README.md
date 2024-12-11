# Sample Rust/Actix-Web

This is a sample **Rust** project using **Actix-Web**, designed to demonstrate a simple and scalable backend web framework. This project showcases features like routing, custom 404 handling, and responsive development environments, making it an excellent starting point for web development using Actix-Web.

---  

## 🚀 Getting Started  

### Open Using Daytona  

1. **Install Daytona**:  
   Follow the [Daytona installation guide](https://www.daytona.io/docs/installation/installation/).  

2. **Create the Workspace**:  
   Use Daytona to create a new workspace from this repository:  

   ```bash  
   daytona create https://github.com/ArnavK-09/sample-rust-actix
   ```
  

3. **Configured Dependencies**:
Once inside the Daytona workspace, dependencies like Rust and Actix-Web will be pre-configured using the provided devcontainer. To manually verify dependencies, ensure you have Rust installed:

   ```bash
   rustup show
   ```


4. **Start the Application**:
The following command to build and launch the Actix-Web server would run on project creation:

   ```bash
   cargo run
   ```

5. **The server will start!**



---

## ✨ Features

### 1. Custom Routing

- A default / route that responds with "Hello world!"

- A /repeat POST route that echoes the request body back to the client.


### 2. Custom 404 Page

- Handles all unknown routes and serves a custom 404.html page to enhance user experience.


### 3. Standardized Development Environment

- Integrated Dev Containers (devcontainer.json) for consistent development setups across machines.

- Pre-configured Visual Studio Code extensions for enhanced productivity.


### 4. Built for Daytona

- Seamless integration with Daytona for quick workspace setup and deployment.



---

## 🛠️ Project Structure

```
├── src/
│   ├── main.rs       # Entry point for the application
├── static/           # Folder containing static files 
├── .devcontainer/    # Pre-configured devcontainer setup
├── Cargo.toml        # Project dependencies
├── README.md         # Project documentation
```

---

### 📖 Learn More

- Actix-Web Documentation: https://actix.rs/

- Rust Programming Language: https://www.rust-lang.org/

- Daytona Workspaces: https://www.daytona.io/




