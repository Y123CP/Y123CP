import os
import platform
import re

from .tree_sitter_runtime import get_rust_language, new_parser, query_captures


class RustParser:

    def __init__(self, root_path: str) -> None:
        self.root_path = root_path
        parser_name = "MAC_rust_parser.so" if platform.system() == "Darwin" else "rust_parser.so"
        self.rust_parser_so_path = os.path.join(root_path, "dependencyLib", parser_name)
        self.RUST_LANGUAGE = get_rust_language()
        self.parser = new_parser(self.RUST_LANGUAGE)

                                                         
    def get_rust_definitions(self, rust_code: str) -> list:
        "" 
                                            
                                                        
           

                    
        tree = self.parser.parse(bytes(rust_code, "utf8"))
        root_node = tree.root_node

        struct_names = []
        function_names = []
        static_names = []
        type_names = []
        enum_names = []
        const_names = []
        union_names = []
        impl_functions = []                     
        impl_names = []
                                   
                                         
                                                                  
        struct_query = """
        (struct_item
        name: (type_identifier) @struct_name)
        """
                                                                
        function_query = """
        (function_item
        name: (identifier) @function_name)
        """
                                                              
        static_query = """
        (static_item
        name: (identifier) @static_name)
        """
                                                               
        type_query = """
        (type_item
        name: (type_identifier) @type_name)
        """
                                                             
        enum_query = """
        (enum_item
        name: (type_identifier) @enum_name)
        """
                                                          
        const_query = """
        (const_item
        name: (identifier) @const_name)
        """
                                                                    
        union_query = """
        (union_item
        name: (type_identifier) @union_name)
        """
        
                        
        all_impl_query = """
        (impl_item) @impl_item
        """
        

                   
        struct_captures = query_captures(
            self.RUST_LANGUAGE, struct_query, root_node
        )
        for node, _ in struct_captures:
            struct_names.append(node.text.decode('utf8'))

        function_captures = query_captures(
            self.RUST_LANGUAGE, function_query, root_node
        )
        for node, _ in function_captures:
                                         
            if not self._is_in_impl_block(node):
                function_names.append(node.text.decode('utf8'))

        static_captures = query_captures(
            self.RUST_LANGUAGE, static_query, root_node
        )
        for node, _ in static_captures:
            static_names.append(node.text.decode('utf8'))

        type_captures = query_captures(
            self.RUST_LANGUAGE, type_query, root_node
        )
        for node, _ in type_captures:
            type_names.append(node.text.decode('utf8'))
            
        enum_captures = query_captures(
            self.RUST_LANGUAGE, enum_query, root_node
        )
        for node, _ in enum_captures:
            enum_names.append(node.text.decode('utf8'))
            
        const_captures = query_captures(
            self.RUST_LANGUAGE, const_query, root_node
        )
        for node, _ in const_captures:
            const_names.append(node.text.decode('utf8'))

        union_captures = query_captures(
            self.RUST_LANGUAGE, union_query, root_node
        )
        for node, _ in union_captures:
            union_names.append(node.text.decode('utf8'))

        all_impl_captures = query_captures(
            self.RUST_LANGUAGE, all_impl_query, root_node
        )
        for node, _ in all_impl_captures:
            type_node = node.child_by_field_name('type')
            trait_node = node.child_by_field_name('trait')
            
            if trait_node and type_node:
                name = f"{trait_node.text.decode('utf8')} for {type_node.text.decode('utf8')}"
                impl_names.append(name)
            elif type_node:
                name = type_node.text.decode('utf8')
                impl_names.append(name)
            else:
                name = "[Anonymous impl]"
                impl_names.append(name)


                              
        all_names = struct_names + function_names + static_names + type_names + enum_names + union_names + impl_names + const_names

        # Fallback: if tree-sitter found nothing but code contains fn signatures,
        # use regex to extract function names (handles syntax-broken code).
        if not all_names and rust_code.strip():
            for ri in RustParser.regex_find_rust_functions(rust_code):
                fn_name = ri.get('name')
                if fn_name:
                    all_names.append(fn_name)

        return all_names
        
    def _is_in_impl_block(self, node):
        "" 
                         
           
        current = node
        while current.parent is not None:
            current = current.parent
            if current.type == 'impl_item':
                return True
        return False

    def _get_impl_type_name(self, node):
        "" 
                            
           
        current = node
        while current.parent is not None:
            current = current.parent
            if current.type == 'impl_item':
                type_node = current.child_by_field_name('type')
                if type_node:
                    return type_node.text.decode('utf8')
                break
        return None

    def has_syntax_error(self, code_string: str) -> bool:
        """True iff tree-sitter reports any ERROR node in ``code_string``.

        Tree-sitter is LENIENT: it will happily return a partial ``items``
        list even when the parse broke (e.g. ``pub macro_rules!`` yields a
        ``macro`` item AND an error node). ``find_rust_items`` cannot be
        used as a syntax gate. Prefer this method when deciding whether a
        patch is safe to write.
        """
        tree = self.parser.parse(bytes(code_string, "utf8"))
        return bool(tree.root_node.has_error)

                                      
    def find_rust_items(self, code_string: str) -> list[dict]:
        "" 
                                            
                                                       
                                                     
           
        
        items = []
        code_bytes = bytes(code_string, "utf8")
        tree = self.parser.parse(code_bytes)
        root_node = tree.root_node

                                                     
        # Different tree-sitter-rust versions may use different names:
        #   newer: extern_block, older: foreign_mod_item
        _QUERY_WITH_EXTERN = """
        [
        (function_item) @item
        (struct_item) @item
        (union_item) @item
        (enum_item) @item
        (impl_item) @item
        (trait_item) @item
        (type_item) @item
        (const_item) @item
        (static_item) @item
        (use_declaration) @item
        (mod_item) @item
        (macro_definition) @item
        ({extern_node}) @item
        ]
        """
        _QUERY_WITHOUT_EXTERN = """
        [
        (function_item) @item
        (struct_item) @item
        (union_item) @item
        (enum_item) @item
        (impl_item) @item
        (trait_item) @item
        (type_item) @item
        (const_item) @item
        (static_item) @item
        (use_declaration) @item
        (mod_item) @item
        (macro_definition) @item
        ]
        """
        # Try extern_block first, then foreign_mod_item, then no extern support
        captures = None
        extern_node_name = None
        for candidate in ['extern_block', 'foreign_mod_item']:
            try:
                qs = _QUERY_WITH_EXTERN.replace('{extern_node}', candidate)
                captures = query_captures(
                    self.RUST_LANGUAGE,
                    qs,
                    root_node,
                )
                extern_node_name = candidate
                break
            except Exception:
                continue
        if captures is None:
            captures = query_captures(
                self.RUST_LANGUAGE,
                _QUERY_WITHOUT_EXTERN,
                root_node,
            )

        for node, _ in captures:
                                 
                                              
                                            
            
            # Skip items that are INSIDE an extern block — we capture the
            # block itself as a single item, so we must not double-count
            # its children (function declarations, static declarations, etc.).
            if extern_node_name and node.type != extern_node_name and node.parent and node.parent.type == extern_node_name:
                continue
            
            start_node = node
            
                                      
            current_node = node
            while current_node.prev_sibling is not None:
                prev_sibling = current_node.prev_sibling
                                            
                if prev_sibling.type in ['attribute_item', 'line_comment', 'block_comment']:
                    start_node = prev_sibling
                    current_node = prev_sibling
                else:
                                     
                    break

                                                       
            start_line = start_node.start_point[0] + 1
            end_line = node.end_point[0] + 1
            
            item_type = node.type
            name = "[N/A]"

                                      
            if item_type == 'function_item':
                name_node = node.child_by_field_name('name')
                if name_node:
                    func_name = name_node.text.decode('utf8')
                                      
                    if self._is_in_impl_block(node):
                                           
                        impl_type_name = self._get_impl_type_name(node)
                        if impl_type_name:
                                                            
                            # name = f"{func_name}-{impl_type_name}"
                            continue
                        else:
                            name = func_name
                    else:
                        name = func_name
                else:
                    name = "[N/A]"
            elif item_type in ['struct_item', 'union_item', 'enum_item', 'trait_item', 'mod_item', 'const_item', 'static_item', 'type_item']:
                name_node = node.child_by_field_name('name')
                if name_node:
                    name = name_node.text.decode('utf8')

            elif item_type == 'macro_definition':
                # `macro_rules! NAME { ... }` — the `name` field holds the
                # identifier.  Reported under type `macro` so callers
                # (read_item / file_editor) treat it like any other symbol.
                name_node = node.child_by_field_name('name')
                if name_node:
                    name = name_node.text.decode('utf8')
            
            elif item_type == 'impl_item':
                type_node = node.child_by_field_name('type')
                trait_node = node.child_by_field_name('trait')
                if trait_node and type_node:
                    name = f"impl {trait_node.text.decode('utf8')} for {type_node.text.decode('utf8')}"
                elif type_node:
                    name = f"impl {type_node.text.decode('utf8')}"
                else:
                    name = "impl [Anonymous]"

            elif item_type == 'use_declaration':
                path_node = node.children[1]
                if path_node:
                    name = path_node.text.decode('utf8')

            elif extern_node_name and item_type == extern_node_name:
                # Name the extern block after its first internal declaration.
                # e.g. extern "C" { pub fn BZ2_bzlibVersion() -> ...; }
                #  → name = "BZ2_bzlibVersion"
                # This lets find_rust_items and file_editor locate it by name.
                first_decl_name = None
                for child in node.children:
                    # Inside extern_block, declarations may be wrapped in
                    # a declaration_list node or appear directly as children.
                    search_nodes = [child]
                    if child.type == 'declaration_list':
                        search_nodes = list(child.children)
                    for decl_node in search_nodes:
                        name_child = decl_node.child_by_field_name('name')
                        if name_child:
                            first_decl_name = name_child.text.decode('utf8')
                            break
                    if first_decl_name:
                        break
                name = first_decl_name if first_decl_name else "extern_block"

            # Normalize extern block type name to 'extern_block' regardless
            # of which grammar variant (extern_block / foreign_mod_item) matched.
            output_type = item_type.replace('_item', '')
            if extern_node_name and item_type == extern_node_name:
                output_type = 'extern_block'
            elif item_type == 'macro_definition':
                output_type = 'macro'

            items.append({
                "type": output_type,
                "name": name,
                "start_line": start_line,
                "end_line": end_line,
            })
            
        return items

    # ── regex-based best-effort function finder (for syntactically broken code) ──

    # Pattern matches:  pub unsafe extern "C" fn name(  /  fn name<T>(  etc.
    _FN_SIG_RE = re.compile(
        r'^[ \t]*'                         # leading whitespace
        r'(?:pub\s+)?'                     # optional pub
        r'(?:unsafe\s+)?'                  # optional unsafe
        r'(?:extern\s+"[^"]*"\s+)?'        # optional extern "C"
        r'fn\s+(\w+)',                     # fn <name>
        re.MULTILINE,
    )

    @staticmethod
    def regex_find_rust_functions(code_string: str) -> list[dict]:
        """Best-effort function finder using regex + brace counting.

        When tree-sitter cannot parse a file (e.g. syntax errors), this
        method provides a fallback that can still locate function boundaries
        by matching ``fn <name>`` signatures and counting ``{`` / ``}`` to
        find the closing brace.

        Returns a list of dicts compatible with ``find_rust_items`` output::

            [{'name': 'foo', 'type': 'function_item',
              'start_line': 10, 'end_line': 120}, ...]
        """
        items: list[dict] = []
        for m in RustParser._FN_SIG_RE.finditer(code_string):
            fn_name = m.group(1)
            # start_line is 1-indexed
            start_line = code_string[:m.start()].count('\n') + 1

            # Find the opening '{' after the signature
            brace_pos = code_string.find('{', m.end())
            if brace_pos == -1:
                continue  # no body — skip (could be a declaration)

            # Count braces to find the matching '}'
            depth = 0
            i = brace_pos
            length = len(code_string)
            in_line_comment = False
            in_block_comment = False
            in_string = False
            in_raw_string = False
            raw_hashes = 0
            escape_next = False

            while i < length:
                ch = code_string[i]

                # Handle escape sequences inside strings
                if escape_next:
                    escape_next = False
                    i += 1
                    continue

                # Line comment
                if in_line_comment:
                    if ch == '\n':
                        in_line_comment = False
                    i += 1
                    continue

                # Block comment (supports nesting)
                if in_block_comment:
                    if ch == '/' and i + 1 < length and code_string[i + 1] == '*':
                        depth_comment = getattr(regex_find_rust_functions, '_bc', 0)
                        # For simplicity, don't track nested block comments here
                        i += 2
                        continue
                    if ch == '*' and i + 1 < length and code_string[i + 1] == '/':
                        in_block_comment = False
                        i += 2
                        continue
                    i += 1
                    continue

                # Raw string: r#"..."#  r##"..."##  etc.
                if in_raw_string:
                    if ch == '"':
                        # Check for closing hashes
                        j = i + 1
                        count = 0
                        while j < length and code_string[j] == '#' and count < raw_hashes:
                            count += 1
                            j += 1
                        if count == raw_hashes:
                            in_raw_string = False
                            i = j
                            continue
                    i += 1
                    continue

                # Regular string
                if in_string:
                    if ch == '\\':
                        escape_next = True
                    elif ch == '"':
                        in_string = False
                    i += 1
                    continue

                # Detect comment / string starts
                if ch == '/' and i + 1 < length:
                    next_ch = code_string[i + 1]
                    if next_ch == '/':
                        in_line_comment = True
                        i += 2
                        continue
                    if next_ch == '*':
                        in_block_comment = True
                        i += 2
                        continue

                # Detect raw string start: r#"  or r##" etc.
                if ch == 'r' and i + 1 < length and code_string[i + 1] in ('"', '#'):
                    j = i + 1
                    hashes = 0
                    while j < length and code_string[j] == '#':
                        hashes += 1
                        j += 1
                    if j < length and code_string[j] == '"':
                        in_raw_string = True
                        raw_hashes = hashes
                        i = j + 1
                        continue

                # Detect regular string
                if ch == '"':
                    in_string = True
                    i += 1
                    continue

                # Detect char literal (skip to avoid counting braces inside)
                if ch == "'" and i + 1 < length:
                    # Simple heuristic: 'x' or '\x'
                    if i + 2 < length and code_string[i + 1] == '\\' and i + 3 < length and code_string[i + 3] == "'":
                        i += 4
                        continue
                    if i + 2 < length and code_string[i + 2] == "'":
                        i += 3
                        continue

                # Brace counting
                if ch == '{':
                    depth += 1
                elif ch == '}':
                    depth -= 1
                    if depth == 0:
                        end_line = code_string[:i + 1].count('\n') + 1
                        items.append({
                            'name': fn_name,
                            'type': 'function_item',
                            'start_line': start_line,
                            'end_line': end_line,
                        })
                        break

                i += 1

        return items

                                                                     
    def extract_item_content(self, code_string: str) -> list[dict]:
        "" 
                                 
                     
                        
                        
           
        items = []
                           
        code_bytes = bytes(code_string, "utf8")
        tree = self.parser.parse(code_bytes)
        root_node = tree.root_node

                      
        query_string = """
        [
        (function_item) @item
        (struct_item) @item
        (enum_item) @item
        (impl_item) @item
        (trait_item) @item
        (mod_item) @item
        ]
        """
        captures = query_captures(
            self.RUST_LANGUAGE,
            query_string,
            root_node,
        )

        for node, _ in captures:
            item_type = node.type.replace('_item', '')
            content = ""

                                   

            if item_type == 'function':
                                              
                body_node = node.child_by_field_name('body')
                
                if body_node:
                                                  
                    signature_end_byte = body_node.start_byte
                    signature_bytes = code_bytes[node.start_byte:signature_end_byte]
                    content = signature_bytes.decode('utf-8').strip()
                else:
                                                       
                    content = node.text.decode('utf-8')

            elif item_type in ['struct', 'enum', 'trait', 'mod', 'impl']:
                                         
                content = node.text.decode('utf-8')
                
            else:
                                
                content = node.text.decode('utf-8')

                        
            name_node = node.child_by_field_name('name')
            name = name_node.text.decode('utf8') if name_node else f"[{item_type}]"
                            
            if item_type == 'impl':
                type_node = node.child_by_field_name('type')
                trait_node = node.child_by_field_name('trait')
                if trait_node and type_node:
                    name = f"impl {trait_node.text.decode('utf8')} for {type_node.text.decode('utf8')}"
                elif type_node:
                    name = f"impl {type_node.text.decode('utf8')}"


            # items.append({
            #     "name": name,
            #     "content": content,
            # })
            items.append(content)
        return_str = "\n".join(items)
        if len(return_str) == 0:
            return_str = code_string  
        return return_str



    def separate_use_statements(self, rust_code: str) -> tuple[list[str], str]:
        "" 
                                          
                                                              
           
              
        code_bytes = bytes(rust_code, "utf8")
        tree = self.parser.parse(code_bytes)
        root_node = tree.root_node
        
                    
        use_query = """
        (use_declaration) @use_decl
        """
        
        use_captures = query_captures(
            self.RUST_LANGUAGE,
            use_query,
            root_node,
        )
        use_ranges = []
        use_statements = []
        
                         
        for node, _ in use_captures:
                          
            start_byte = node.start_byte
            end_byte = node.end_byte
            
                          
            use_text = code_bytes[start_byte:end_byte].decode('utf8')
            use_statements.append(use_text)
            
                          
            use_ranges.append((start_byte, end_byte))
        
                                  
        use_ranges.sort(key=lambda x: x[0], reverse=True)
        
                      
        remaining_code_bytes = bytearray(code_bytes)
        for start_byte, end_byte in use_ranges:
                                 
            use_length = end_byte - start_byte
            remaining_code_bytes[start_byte:end_byte] = b' ' * use_length
        
                        
        remaining_code = remaining_code_bytes.decode('utf8')
        
                            
        lines = remaining_code.split('\n')
        cleaned_lines = []
        prev_empty = False
        
        for line in lines:
            is_empty = not line.strip()
            if is_empty and prev_empty:
                continue           
            cleaned_lines.append(line)
            prev_empty = is_empty
        
        remaining_code = '\n'.join(cleaned_lines).strip()
        
        return use_statements, remaining_code

    def separate_use_statements_with_lines(self, rust_code: str) -> tuple[list[str], list[str]]:
        "" 
                                                  
                                             
           
        use_statements, remaining_code = self.separate_use_statements(rust_code)
        
                    
        use_lines = []
        for use_stmt in use_statements:
            use_lines.extend(use_stmt.split('\n'))
        
                        
        code_lines = [line for line in remaining_code.split('\n') if line.strip()]
        
        return use_lines, code_lines
    
    
if __name__ == "__main__":
    import json
    import os
    
    root_path = "/home/usr/CodeTrans_c2Rust"
    rust_parser = RustParser(root_path)
    
    # file_path = "/home/usr/CodeTrans_c2Rust/dataset/PA_trans_projects/binn/src/common/binn_mod.rs"
    # with open(file_path, 'r', encoding='utf-8') as f:
    #     code_string = f.read()
    code_string = '''
    use crate::{
    JsonArray, JsonArrayElement, JsonObject, JsonObjectElement, JsonParseError, JsonParseFlags,
    JsonParseState, JsonValue, JsonValueEx,
};
use crate::{json_get_key_size, json_get_number_size, json_get_string_size, json_skip_all_skippables};

pub fn json_get_value_size(state: Option<&mut JsonParseState>, is_global_object: usize) -> usize {
    if let Some(state) = state {
        let flags_bitset = state.flags_bitset;
        let src = state.src.as_ref().map(|s| &s[..]).unwrap_or(&[]);
        let size = state.size;

        if flags_bitset & JsonParseFlags::AllowLocationInformation as usize != 0 {
            state.dom_size += std::mem::size_of::<JsonValueEx>();
        } else {
            state.dom_size += std::mem::size_of::<JsonValue>();
        }

        if is_global_object != 0 {
            return json_get_object_size(Some(state), 1);
        } else {
            if json_skip_all_skippables(Some(state)) != 0 {
                state.error = JsonParseError::PrematureEndOfBuffer as usize;
                return 1;
            }

            let offset = state.offset;
            if offset >= size {
                state.error = JsonParseError::PrematureEndOfBuffer as usize;
                return 1;
            }

            match src[offset] as char {
                '"' => return json_get_string_size(Some(state), 0),
                '\'' => {
                    if flags_bitset & JsonParseFlags::AllowSingleQuotedStrings as usize != 0 {
                        return json_get_string_size(Some(state), 0);
                    } else {
                        state.error = JsonParseError::InvalidValue as usize;
                        return 1;
                    }
                }
                '{' => return json_get_object_size(Some(state), 0),
                '[' => return json_get_array_size(Some(state)),
                '-' | '0'..='9' => return json_get_number_size(Some(state)),
                '+' => {
                    if flags_bitset & JsonParseFlags::AllowLeadingPlusSign as usize != 0 {
                        return json_get_number_size(Some(state));
                    } else {
                        state.error = JsonParseError::InvalidNumberFormat as usize;
                        return 1;
                    }
                }
                '.' => {
                    if flags_bitset & JsonParseFlags::AllowLeadingOrTrailingDecimalPoint as usize != 0
                    {
                        return json_get_number_size(Some(state));
                    } else {
                        state.error = JsonParseError::InvalidNumberFormat as usize;
                        return 1;
                    }
                }
                _ => {
                    if offset + 4 <= size
                        && &src[offset..offset + 4] == b"true"
                    {
                        state.offset += 4;
                        return 0;
                    } else if offset + 5 <= size
                        && &src[offset..offset + 5] == b"false"
                    {
                        state.offset += 5;
                        return 0;
                    } else if offset + 4 <= size
                        && &src[offset..offset + 4] == b"null"
                    {
                        state.offset += 4;
                        return 0;
                    } else if flags_bitset & JsonParseFlags::AllowInfAndNan as usize != 0 {
                        if offset + 3 <= size && &src[offset..offset + 3] == b"NaN" {
                            return json_get_number_size(Some(state));
                        } else if offset + 8 <= size
                            && &src[offset..offset + 8] == b"Infinity"
                        {
                            return json_get_number_size(Some(state));
                        }
                    }
                    state.error = JsonParseError::InvalidValue as usize;
                    return 1;
                }
            }
        }
    }
    1
}

pub fn json_get_object_size(
    state: Option<&mut JsonParseState>,
    mut is_global_object: usize,
) -> usize {
    if let Some(state) = state {
        let flags_bitset = state.flags_bitset;
        let src = state.src.as_ref().map(|s| &s[..]).unwrap_or(&[]);
        let size = state.size;
        let mut elements = 0;
        let mut allow_comma = false;
        let mut found_closing_brace = false;

        if is_global_object != 0 {
            if json_skip_all_skippables(Some(state)) == 0
                && state.offset < size
                && src[state.offset] as char == '{'
            {
                is_global_object = 0;
            }
        }

        if is_global_object == 0 {
            if state.offset >= size || src[state.offset] as char != '{' {
                state.error = JsonParseError::Unknown as usize;
                return 1;
            }
            state.offset += 1;
        }

        state.dom_size += std::mem::size_of::<JsonObject>();

        if state.offset == size && is_global_object == 0 {
            state.error = JsonParseError::PrematureEndOfBuffer as usize;
            return 1;
        }

        while state.offset < size {
            if is_global_object == 0 {
                if json_skip_all_skippables(Some(state)) != 0 {
                    state.error = JsonParseError::PrematureEndOfBuffer as usize;
                    return 1;
                }

                if src[state.offset] as char == '}' {
                    state.offset += 1;
                    found_closing_brace = true;
                    break;
                }
            } else {
                if json_skip_all_skippables(Some(state)) != 0 {
                    break;
                }
            }

            if allow_comma {
                if src[state.offset] as char == ',' {
                    state.offset += 1;
                    allow_comma = false;
                } else if flags_bitset & JsonParseFlags::AllowNoCommas as usize == 0 {
                    state.error = JsonParseError::ExpectedCommaOrClosingBracket as usize;
                    return 1;
                }

                if flags_bitset & JsonParseFlags::AllowTrailingComma as usize != 0 {
                    continue;
                } else {
                    if json_skip_all_skippables(Some(state)) != 0 {
                        state.error = JsonParseError::PrematureEndOfBuffer as usize;
                        return 1;
                    }
                }
            }

            if json_get_key_size(Some(state)) != 0 {
                state.error = JsonParseError::InvalidString as usize;
                return 1;
            }

            if json_skip_all_skippables(Some(state)) != 0 {
                state.error = JsonParseError::PrematureEndOfBuffer as usize;
                return 1;
            }

            if flags_bitset & JsonParseFlags::AllowEqualsInObject as usize != 0 {
                let current = src[state.offset] as char;
                if current != ':' && current != '=' {
                    state.error = JsonParseError::ExpectedColon as usize;
                    return 1;
                }
            } else {
                if src[state.offset] as char != ':' {
                    state.error = JsonParseError::ExpectedColon as usize;
                    return 1;
                }
            }

            state.offset += 1;

            if json_skip_all_skippables(Some(state)) != 0 {
                state.error = JsonParseError::PrematureEndOfBuffer as usize;
                return 1;
            }

            if json_get_value_size(Some(state), 0) != 0 {
                return 1;
            }

            elements += 1;
            allow_comma = true;
        }

        if state.offset == size && is_global_object == 0 && !found_closing_brace {
            state.error = JsonParseError::PrematureEndOfBuffer as usize;
            return 1;
        }

        state.dom_size += std::mem::size_of::<JsonObjectElement>() * elements;
        return 0;
    }
    1
}

pub fn json_get_array_size(state: Option<&mut JsonParseState>) -> usize {
    if let Some(state) = state {
        let flags_bitset = state.flags_bitset;
        let src = state.src.as_ref().map(|s| &s[..]).unwrap_or(&[]);
        let size = state.size;
        let mut elements = 0;
        let mut allow_comma = false;

        if state.offset >= size || src[state.offset] as char != '[' {
            state.error = JsonParseError::Unknown as usize;
            return 1;
        }

        state.offset += 1;
        state.dom_size += std::mem::size_of::<JsonArray>();

        while state.offset < size {
            if json_skip_all_skippables(Some(state)) != 0 {
                state.error = JsonParseError::PrematureEndOfBuffer as usize;
                return 1;
            }

            if src[state.offset] as char == ']' {
                state.offset += 1;
                state.dom_size += std::mem::size_of::<JsonArrayElement>() * elements;
                return 0;
            }

            if allow_comma {
                if src[state.offset] as char == ',' {
                    state.offset += 1;
                    allow_comma = false;
                } else if flags_bitset & JsonParseFlags::AllowNoCommas as usize == 0 {
                    state.error = JsonParseError::ExpectedCommaOrClosingBracket as usize;
                    return 1;
                }

                if flags_bitset & JsonParseFlags::AllowTrailingComma as usize != 0 {
                    allow_comma = false;
                    continue;
                } else {
                    if json_skip_all_skippables(Some(state)) != 0 {
                        state.error = JsonParseError::PrematureEndOfBuffer as usize;
                        return 1;
                    }
                }
            }

            if json_get_value_size(Some(state), 0) != 0 {
                return 1;
            }

            elements += 1;
            allow_comma = true;
        }

        state.error = JsonParseError::PrematureEndOfBuffer as usize;
        return 1;
    }
    1
}
'''
    # use_stmt, code_stmt = rust_parser.separate_use_statements_with_lines(code_string)
    fixed_items = rust_parser.find_rust_items(code_string)
    print(fixed_items)
