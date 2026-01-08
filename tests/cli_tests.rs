use andiamo::cli::Cli;

#[test]
fn test_cli_default_values() {
    let cli = Cli {
        init: false,
        add_remotes: false,
        push: false,
    };

    assert_eq!(cli.init, false);
    assert_eq!(cli.add_remotes, false);
    assert_eq!(cli.push, false);
}

#[test]
fn test_cli_with_init() {
    let cli = Cli {
        init: true,
        add_remotes: false,
        push: false,
    };

    assert_eq!(cli.init, true);
    assert_eq!(cli.add_remotes, false);
    assert_eq!(cli.push, false);
}

#[test]
fn test_cli_with_all_flags() {
    let cli = Cli {
        init: true,
        add_remotes: true,
        push: true,
    };

    assert_eq!(cli.init, true);
    assert_eq!(cli.add_remotes, true);
    assert_eq!(cli.push, true);
}

#[test]
fn test_cli_partial_flags() {
    let cli = Cli {
        init: false,
        add_remotes: true,
        push: true,
    };

    assert_eq!(cli.init, false);
    assert_eq!(cli.add_remotes, true);
    assert_eq!(cli.push, true);
}

#[test]
fn test_cli_equality() {
    let cli1 = Cli {
        init: true,
        add_remotes: false,
        push: false,
    };

    let cli2 = Cli {
        init: true,
        add_remotes: false,
        push: false,
    };

    assert_eq!(cli1, cli2);
}

#[test]
fn test_cli_clone() {
    let cli1 = Cli {
        init: true,
        add_remotes: true,
        push: false,
    };

    let cli2 = cli1.clone();

    assert_eq!(cli1, cli2);
}

#[test]
fn test_cli_debug() {
    let cli = Cli {
        init: true,
        add_remotes: false,
        push: true,
    };

    let debug_str = format!("{:?}", cli);
    assert!(debug_str.contains("init: true"));
    assert!(debug_str.contains("add_remotes: false"));
    assert!(debug_str.contains("push: true"));
}
