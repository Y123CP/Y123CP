import os
import subprocess
import logging

class GitManager:

    # def __init__(self, root_path: str) -> None:
    #     self.root_path = root_path
    #     self.rust_parser_so_path = os.path.join(root_path, "dependencyLib/rust_parser.so")

    def ensure_git_repo(self, repo_path):
        ""                            
                      
        git_dir = os.path.join(repo_path, '.git')
        if not os.path.exists(git_dir):
                      
            try:
                current_dir = os.getcwd()
                os.chdir(repo_path)
                
                          
                subprocess.run(
                    ["git", "init"],
                    check=True,
                    capture_output=True,
                    text=True
                )
                
                           
                subprocess.run(
                    ["git", "add", "."],
                    check=True,
                    capture_output=True,
                    text=True
                )
                subprocess.run(
                    ["git", "commit", "-m", "Initial commit"],
                    check=True,
                    capture_output=True,
                    text=True
                )
                
                os.chdir(current_dir)
                return True
            except subprocess.CalledProcessError as e:
                if current_dir != os.getcwd():
                    os.chdir(current_dir)
                return False
        return True

    def git_save_state(self, repo_path, message):
        ""                          
        try:
            current_dir = os.getcwd()
            os.chdir(repo_path)
            
                    
            subprocess.run(
                ["git", "add", "."],
                check=True,
                capture_output=True,
                text=True
            )
            
                  
            result = subprocess.run(
                ["git", "commit", "-m", message],
                capture_output=True,
                text=True
            )
            
                               
            if result.returncode != 0:
                                    
                output_text = result.stdout + result.stderr
                if "nothing to commit" in output_text or "working tree clean" in output_text:
                    logging.info("当前工作区无变更，使用现有 HEAD 作为保存点")
                                  
                    result = subprocess.run(
                        ["git", "rev-parse", "HEAD"],
                        check=True,
                        capture_output=True,
                        text=True
                    )
                    commit_hash = result.stdout.strip()
                    os.chdir(current_dir)
                    return commit_hash
                else:
                    logging.warning(f"Git提交失败: {result.stderr}")
                    os.chdir(current_dir)
                    return None
            
                      
            result = subprocess.run(
                ["git", "rev-parse", "HEAD"],
                check=True,
                capture_output=True,
                text=True
            )
            commit_hash = result.stdout.strip()
            
            logging.info(f"已创建Git提交: {commit_hash[:8]} - {message}")
            
            os.chdir(current_dir)
            return commit_hash
        except subprocess.CalledProcessError as e:
            logging.error(f"Git保存状态失败: {e}")
            if current_dir != os.getcwd():
                os.chdir(current_dir)
            return None

    def git_restore_state(self, repo_path, commit_hash):
        ""               
        try:
            current_dir = os.getcwd()
            os.chdir(repo_path)
            
                        
            subprocess.run(
                ["git", "reset", "--hard", commit_hash],
                check=True,
                capture_output=True,
                text=True
            )
            
            logging.info(f"已回退到Git提交: {commit_hash[:8]}")
            
            os.chdir(current_dir)
            return True
        except subprocess.CalledProcessError as e:
            logging.error(f"Git回退状态失败: {e}")
            if current_dir != os.getcwd():
                os.chdir(current_dir)
            return False

    def git_diff_file(self, repo_path: str, file_rel_path: str, context_lines: int = 1, ignore_space: bool = True) -> str:
        ""                                    
                                                                           
        try:
            cmd = ["git", "diff", f"-U{context_lines}"]
            if ignore_space:
                cmd.append("-w")
            cmd.extend(["--", file_rel_path])
            
            result = subprocess.run(
                cmd,
                cwd=repo_path,
                capture_output=True,
                text=True,
                timeout=10
            )
            return result.stdout.strip()
        except Exception as e:
            logging.warning(f"git diff failed for {file_rel_path}: {e}")
            return ""

    def git_diff_strings(self, old_text: str, new_text: str, context_lines: int = 1, ignore_space: bool = True) -> str:
        ""                                     
                                                  
        import tempfile
        try:
            with tempfile.NamedTemporaryFile('w', suffix='.rs', delete=False) as f1, \
                 tempfile.NamedTemporaryFile('w', suffix='.rs', delete=False) as f2:
                f1.write(old_text)
                f2.write(new_text)
                p1, p2 = f1.name, f2.name
            
            cmd = ["git", "diff", "--no-index", f"-U{context_lines}"]
            if ignore_space:
                cmd.append("-w")
            cmd.extend([p1, p2])
            
            res = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
            out = res.stdout.strip()
            
            os.remove(p1)
            os.remove(p2)
            
            if not out:
                return ""
            
            # Clean up the output headers
            cleaned = []
            for line in out.splitlines():
                if line.startswith("diff --git") or line.startswith("index "):
                    continue
                if line.startswith("--- "):
                    cleaned.append("--- Rust (Before Changes)")
                elif line.startswith("+++ "):
                    cleaned.append("+++ Rust (After Changes)")
                else:
                    cleaned.append(line)
                    
            return "\n".join(cleaned)
        except Exception as e:
            logging.warning(f"git_diff_strings failed: {e}")
            return ""