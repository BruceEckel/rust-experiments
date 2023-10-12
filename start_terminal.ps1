# Doesn't work yet, tried to use ChatGPT to generate this

# Define the paths and tab names
$tab1_path = "C:\git\python-experiments\concurrent_failure"
$tab1_name = "Python"

$tab2_path = "C:\git\rust-experiments\concurrent_failure\multiple_tasks"
$tab2_name = "Rust"

# Construct the command
$wtCommand = "wt new-tab -d '$tab1_path' -n '$tab1_name' ; new-tab -d '$tab2_path' -n '$tab2_name' ;"

# Launch Windows Terminal with the specified configuration
Invoke-Expression $wtCommand
