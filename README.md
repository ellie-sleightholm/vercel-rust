# Rust Vercel API

## Introduction
This project is a Rust API deployed on Vercel. It is designed for users to build Rust code which can then be called using POST requests. This repository is an easy to use Rust-Vercel API that can be amended for your own requirements.

## Example 
There are 3 main API calls in the repository and they are outlined below.

1) A basic indexing call:
    ```http
    POST https://vercel-rust-two.vercel.app/api/user/[1]
    Content-Type: application/json

    {}
    ```

    This returns the number received at the end of the url. In this case we have `1`.
    ```bash
    [
    1
    ]
    ```

2) A random planet generator.
    ```http
    POST https://vercel-rust-two.vercel.app/api/simple
    Content-Type: application/json

    {}
    ```
    This returns a randomly generated planet in our Solar System.
    ```bash
    {
        "message": "I choose the planet, Earth!"
    }
    ```

3) A random planet generator with a specified input.
    ```http
    POST https://vercel-rust-two.vercel.app/api/complex
    Content-Type: application/json

    {
        "name": "Ellie"
    }
    ```
    This returns a randomly generated planet in our Solar System.
    ```bash
    {
        "message": "Ellie says: I choose the planet, Uranus!"
    }
    ```

## Using this Rust API for Your Own Projects
This repository can be cloned and used for your own personal projects. To clone the repository:
```bash
git clone https://github.com/ellie-sleightholm/vercel-rust.git
```

Once cloned, you can use the contents of this repository in your own personal repository. It is important that you have your repository linked to Vercel ready for deployment. Then you're ready to go! Make changes and push to GitHub which will create your own personal URL. You can use this when making POST requests (see [here](request.http).)