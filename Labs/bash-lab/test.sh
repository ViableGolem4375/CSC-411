echo Hello...
file="test2.txt"
if [ -f $file ] ; then
	echo "exists!"
else
	echo "does not exist!"
fi
chmod u+x ./src/main.rs
function_name () {
	echo The function ran!
}
function_name
run_fgroups () {
	rustc ./src/main.rs
}
run_fgroups
