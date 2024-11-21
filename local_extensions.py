from cookiecutter.utils import simple_filter


@simple_filter
def strip_prefix(v, prefix, *args):
    all_prefixes = []
    all_prefixes.append(prefix)
    all_prefixes.extend(args)

    for p in all_prefixes:
        if v.startswith(p):
            return v[len(p):]

    return v
