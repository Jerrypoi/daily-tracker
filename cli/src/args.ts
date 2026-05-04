// Minimal arg parser: supports positionals, --flag value, --flag=value, and
// boolean --flag. Unknown flags pass through into the flags map. Stops
// gathering positionals at the first `--`, after which everything is treated
// as positional.

export type FlagValue = string | true;
export interface ParsedArgs {
  positional: string[];
  flags: Record<string, FlagValue>;
}

export function parseArgs(argv: string[]): ParsedArgs {
  const positional: string[] = [];
  const flags: Record<string, FlagValue> = {};
  let i = 0;
  let passthrough = false;

  while (i < argv.length) {
    const a = argv[i]!;

    if (passthrough) {
      positional.push(a);
      i++;
      continue;
    }

    if (a === "--") {
      passthrough = true;
      i++;
      continue;
    }

    if (a.startsWith("--")) {
      const eq = a.indexOf("=");
      if (eq !== -1) {
        flags[a.slice(2, eq)] = a.slice(eq + 1);
        i++;
        continue;
      }
      const key = a.slice(2);
      const next = argv[i + 1];
      if (next !== undefined && !next.startsWith("--")) {
        flags[key] = next;
        i += 2;
      } else {
        flags[key] = true;
        i++;
      }
      continue;
    }

    if (a.startsWith("-") && a.length > 1) {
      // Short flags: only -h supported as alias for --help.
      const key = a.slice(1);
      flags[key === "h" ? "help" : key] = true;
      i++;
      continue;
    }

    positional.push(a);
    i++;
  }

  return { positional, flags };
}
