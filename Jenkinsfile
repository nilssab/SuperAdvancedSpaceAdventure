pipeline {
	agent master
	stages {
		stage("build") {
			steps{
				echo "building ${BRANCH_NAME}"
				sh "cargo build"
			}
		}
	}
}
