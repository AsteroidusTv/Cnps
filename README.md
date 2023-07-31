# CNPS-Rust


CNPS (Create new project simply) is a powerful tool built in Rust that simplifies the process of creating projects in various programming languages. Currently, CNPS supports project generation for Rust, Python, Go, Assembly, and HTML (with or without JavaScript).
With CNPS, you can create new projects with ease, and it offers a range of customizable options to suit your workflow. Upon configuration, you can specify the target directory where your projects will be generated. Additionally, CNPS allows you to choose whether to organize your projects into subdirectories based on their respective programming languages.
One of the key features of CNPS is its seamless integration with popular Integrated Development Environments (IDEs). During the setup, you get to select your preferred IDE from a list of supported options, such as Sublime Text or Visual Studio Code (for the moment). CNPS will automatically open your chosen IDE, ready for you to start working on your newly created project.
For HTML projects, CNPS goes above and beyond by providing a fully-prepared HTML file and accompanying CSS file. The HTML file is pre-filled with a basic template, including essential tags like <!DOCTYPE html>, <html>, <head>, and <body>. Additionally, if JavaScript was selected during configuration, the template will include a link to the script file in the body. This saves you valuable time, allowing you to jump straight into writing your web page's content and functionality.
Moreover, CNPS offers a set of pre-designed layout templates for common use cases, such as a simple one-column layout, a responsive two-column layout, or a flexbox-based grid layout. These layout templates are included in the CSS file, making it effortless to structure and style your web page according to your preferences.
CNPS also establishes the connection between the HTML and CSS files, ensuring they are linked correctly, so you can focus on crafting an aesthetically pleasing and functional web page.
Furthermore, CNPS takes care of opening the newly created HTML file in your default web browser, enabling you to view your page instantly and streamline the development process.
CNPS was previously implemented in [Bash](https://github.com/AsteroidusTv/Create-New-Project-simply), but its transition to Rust brings numerous advantages, including improved performance, enhanced reliability, and a more maintainable codebase.

## Features:

* Create projects in multiple programming languages: Rust, Python, Go, Assembly, and HTML. (more will be added in the future)
* Choose project directory and optional subdirectories based on language.
* Seamless integration with popular IDEs like Sublime Text and Visual Studio Code. (more will be added in the future)
* Automatic pre-filling of HTML and CSS templates for faster setup.
* Pre-designed layout templates for easy styling of web pages.
* Automatic connection between HTML and CSS files.
* Automatic opening of the web page in your default browser for immediate preview.
* Faster and more reliable than the previous Bash implementation.


## Dependencies

### For using cnps with rust:
```
sudo apt install cargo
```
###  For using cnps with go:
```
sudo apt install golang
```
You dont need to install anything for other languages.


## Install

### Download the deb package
### Install it:
```
sudo dpkg -i cnps.deb
```
With the packag cnps.deb when you install it a .desktop application will be created as well as a personalized command "newp"
