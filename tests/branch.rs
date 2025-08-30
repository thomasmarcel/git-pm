use git_pm::sanitize_branch;

#[test]
fn test_sanitize_branch_basic() {
    let input = "MINOI-31 Zoom d'images dans galerie";
    let output = sanitize_branch(input);
    assert_eq!(output, "MINOI-31-zoom-d-images-dans-galerie");
}

#[test]
fn test_sanitize_branch_with_symbols() {
    let input = "PROJ-42 Fixé: bug étrange!!";
    let output = sanitize_branch(input);
    assert_eq!(output, "PROJ-42-fixe-bug-etrange");
}

#[test]
fn test_sanitize_branch_only_taskid() {
    let input = "TASK-99";
    let output = sanitize_branch(input);
    assert_eq!(output, "TASK-99");
}
