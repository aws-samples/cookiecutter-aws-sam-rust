# Cookiecutter SAM template for Lambda functions in Rust

This is a [Cookiecutter](https://github.com/cookiecutter/cookiecutter) template to create a serverless application based on the Serverless Application Model (SAM) and Rust.

It is important to note that you should not try to `git clone` this project but use `SAM` CLI instead as ``{{cookiecutter.project_slug}}`` will be rendered based on your input and therefore all variables and files will be rendered properly.

## Usage

Generate a new SAM based Serverless App: `sam init --location gh:aws-samples/cookiecutter-aws-sam-rust`

You'll be prompted a few questions to help this cookiecutter template to scaffold this project and after its completed you should see a new folder at your current path with the name of the project you gave as input.

## Security

See [CONTRIBUTING](CONTRIBUTING.md#security-issue-notifications) for more information.

## License

This library is licensed under the MIT-0 License. See the LICENSE file.
