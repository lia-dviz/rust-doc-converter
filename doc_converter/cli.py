import click
from .core import process_document

@click.command()
@click.option("--input", "-i", required=True, type=click.Path(exists=True))
@click.option("--format", "-f", default="markdown")
@click.option("--bundle", "-b", type=click.Path())
@click.option("--output", "-o", type=click.Path())
def cli(input, format, bundle, output):
    result = process_document(input, format, bundle)
    if output:
        with open(output, "w", encoding="utf-8") as f:
            f.write(result)
    else:
        click.echo(result)
