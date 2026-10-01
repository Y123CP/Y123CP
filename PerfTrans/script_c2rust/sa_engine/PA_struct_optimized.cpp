                                         
// ================================================
                     
                   
                 
// ==========================================================================================
          
                          
                                           
// - Removed BFS depth limits
               
                                 
// ==========================================================================================
#include "DDA/ContextDDA.h"
#include "DDA/DDAClient.h"
#include "SABER/SaberCheckerAPI.h"
#include "Util/SVFUtil.h"
#include "WPA/Andersen.h"
#include "MemoryModel/PointerAnalysisImpl.h"
#include "SVF-LLVM/LLVMUtil.h"
#include "SVF-LLVM/SVFIRBuilder.h"
#include "SVFIR/SVFFileSystem.h"
#include "Util/Options.h"
#include <fstream>
#include <iostream>
#include <map>
#include <nlohmann/json.hpp>
#include <set>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace llvm;
using namespace std;
using namespace SVF;
std::map<std::string, std::vector<std::string>> irContents;

/// Function parameter alias analysis client
class StructAnalyzer : public DDAClient {
private:
  // Store SVFIR reference
  SVFIR *pag;

  struct NodeInfo {
    NodeID id;
    std::string type;               
    std::string description;        
    bool hasValue;                      
    std::string valueName;             
    std::string valueType;             
    int line;
    std::string file_name;             
  };
                 
  struct StructInfo {
    std::string name;                                   
    std::map<unsigned, std::string> fieldNames;                
    std::map<unsigned, std::string> fieldTypes;                
    std::set<unsigned> pointerFields;               

    std::map<unsigned, std::map<NodeID, std::vector<std::vector<NodeID>>>>
        fieldUsageNodeGroups;
                               

    std::map<unsigned, bool> fieldNullability;                     
    std::map<unsigned, std::string>
        fieldOwnership;                   
    std::map<unsigned, bool> fieldMutable;                
  };

                                                  
                                                         
                                      
  struct CachedData {
                          
    std::unordered_set<NodeID> modifyingCallArgNodes;

                 
    std::unordered_map<NodeID, NodeInfo> nodeInfoCache;
  };

  CachedData cachedData;

                  
  std::map<StructType *, StructInfo> structInfoMap;

                                                               
                                        
  AndersenWaveDiff *ander = nullptr;
                                        
  std::map<std::pair<StructType *, unsigned>, std::vector<NodeID>>
      fieldAddrIndex;
                                                
  std::map<NodeID, std::pair<StructType *, unsigned>> nodeToField;
                                               
                                          
  std::map<NodeID, std::vector<const SVFFunction *>> indCalleeOf;
                                                      
                                 
  std::set<std::pair<StructType *, unsigned>> freedFields;

public:
         
  StructAnalyzer(SVFIR *p) : DDAClient(p->getModule()), pag(p) {
    collectStructInfo();
  }

  virtual ~StructAnalyzer() {}

             
  inline SVFIR *getPAG() const { return pag; }

                
  void buildModifyingCallIndex() {
    std::cout << "  构建修改型函数调用索引..." << std::endl;

    for (SVFStmt::PEDGEK kind = SVFStmt::Addr; kind <= SVFStmt::ThreadJoin;
         kind = (SVFStmt::PEDGEK)(kind + 1)) {
      for (const SVFStmt *stmt : pag->getSVFStmtSet(kind)) {
        if (!stmt)
          continue;

        std::string stmtStr = stmt->toString();
        if (!isCallToModifyingFunction(stmtStr))
          continue;

        if (kind == SVFStmt::Call) {
          const CallPE *callStmt = SVFUtil::dyn_cast<CallPE>(stmt);
          if (!callStmt)
            continue;

          NodeID argNodeID = callStmt->getRHSVarID();
          cachedData.modifyingCallArgNodes.insert(argNodeID);
        }
      }
    }

    std::cout << "  修改型函数调用索引构建完成，共 "
              << cachedData.modifyingCallArgNodes.size() << " 个参数节点"
              << std::endl;
  }

                   
  void collectStructInfo() {
    std::cout << "收集结构体信息..." << std::endl;

               
    LLVMModuleSet *moduleSet = LLVMModuleSet::getLLVMModuleSet();

                  
    std::set<std::string> excludePrefixes = {
        "llvm.", "_IO_", "struct._IO_", "std::", "__", "pthread_", "FILE"};

             
    for (u32_t i = 0; i < moduleSet->getModuleNum(); ++i) {
      Module *module = moduleSet->getModule(i);

                   
      for (Type *type : module->getIdentifiedStructTypes()) {
        StructType *structType = dyn_cast<StructType>(type);
        if (!structType)
          continue;

        StructInfo structInfo;
        structInfo.name = structType->getName().str();

                            
        bool shouldSkip = false;
        for (const auto &prefix : excludePrefixes) {
          if (structInfo.name.find(prefix) == 0) {
            shouldSkip = true;
            break;
          }
        }

                           
        if (!shouldSkip &&
            (structInfo.name.find("marker") != std::string::npos ||
             structInfo.name.find("wide_data") != std::string::npos ||
             structInfo.name.find("codecvt") != std::string::npos)) {
          shouldSkip = true;
        }

        if (shouldSkip)
          continue;

                   
        for (unsigned idx = 0; idx < structType->getNumElements(); ++idx) {
          Type *elemType = structType->getElementType(idx);
          std::string elemTypeStr = getTypeString(elemType);

          structInfo.fieldTypes[idx] = elemTypeStr;

                        
          if (elemType->isPointerTy()) {
            structInfo.pointerFields.insert(idx);
          }
        }

                  
        structInfoMap[structType] = structInfo;
      }
    }

    std::cout << "结构体信息收集完成，共发现 " << structInfoMap.size()
              << " 个结构体" << std::endl;
  }

                 
  void start_analyzeStructsWithPointerFields(AndersenWaveDiff *ander_) {
    std::cout << "分析带有指针字段的结构体..." << std::endl;
    ander = ander_;

                                                 
                                                         
                                                 
    buildModifyingCallIndex();
                                                  
                          
    buildFieldAddrIndex();
                                          
    buildIndirectCalleeMap();
                                                          
    markFreedFields();

                      
    size_t structCount = structInfoMap.size();
    size_t currentStructIndex = 0;
    for (auto &pair : structInfoMap) {
      currentStructIndex++;
      StructType *structType = pair.first;
      StructInfo &structInfo = pair.second;

               
      std::cout << "处理结构体 [" << currentStructIndex << "/" << structCount
                << "]: " << structInfo.name << std::endl;

                     
      if (structInfo.pointerFields.empty())
        continue;

      std::cout << "分析结构体: " << structInfo.name << std::endl;

                          
      for (unsigned fieldIdx : structInfo.pointerFields) {
                                                                   
        structInfo.fieldNullability[fieldIdx] = true;

        const std::vector<NodeID> &addrNodes =
            fieldAddrNodes(structType, fieldIdx);

                                                          
                                                                  
        if (isFunctionPointerType(structType->getElementType(fieldIdx))) {
          structInfo.fieldOwnership[fieldIdx] = "Borrowed";
        } else {
          structInfo.fieldOwnership[fieldIdx] =
              computeFieldOwnership(structType, fieldIdx, addrNodes)
                  ? "Owning"
                  : "Borrowed";
        }

                                                      
        structInfo.fieldMutable[fieldIdx] =
            analyzeFieldMutability(structType, fieldIdx, addrNodes);
      }
    }
  }

                                                                 

                                                 
  const std::vector<NodeID> &fieldAddrNodes(StructType *structType,
                                            unsigned fieldIdx) {
    static const std::vector<NodeID> kEmpty;
    auto it = fieldAddrIndex.find({structType, fieldIdx});
    return it == fieldAddrIndex.end() ? kEmpty : it->second;
  }

                                            
                                                 
                                                
                      
  void buildFieldAddrIndex() {
    std::cout << "  构建字段寻址节点索引..." << std::endl;
    LLVMModuleSet *ms = LLVMModuleSet::getLLVMModuleSet();
    for (auto nodeIt = pag->begin(); nodeIt != pag->end(); ++nodeIt) {
      const PAGNode *node = nodeIt->second;
      if (!node || !node->hasValue())
        continue;
      const SVFValue *sval = node->getValue();
      if (!sval)
        continue;
      const Value *v = ms->getLLVMValue(sval);
      if (!v)
        continue;
      const GetElementPtrInst *gep = dyn_cast<GetElementPtrInst>(v);
      if (!gep)
        continue;
      StructType *st = dyn_cast<StructType>(gep->getSourceElementType());
      if (!st || structInfoMap.find(st) == structInfoMap.end())
        continue;
      if (gep->getNumIndices() < 2)
        continue;
      const ConstantInt *idx = dyn_cast<ConstantInt>(gep->getOperand(2));
      if (!idx)
        continue;
      unsigned fld = (unsigned)idx->getZExtValue();
      fieldAddrIndex[{st, fld}].push_back(nodeIt->first);
      nodeToField[nodeIt->first] = {st, fld};
    }
    std::cout << "  字段寻址节点索引构建完成，共 " << fieldAddrIndex.size()
              << " 个 (结构体,字段) 项" << std::endl;
  }

                                              
                                                              
                                                       
                                          
  void buildIndirectCalleeMap() {
    unsigned nCS = 0, nMapped = 0;
    for (const auto &kv : ander->getIndCallMap()) {
      const CallICFGNode *cs = kv.first;
      ++nCS;
      const RetICFGNode *ret = cs->getRetICFGNode();
      if (!ret || !ret->getActualRet())
        continue;
      NodeID rid = ret->getActualRet()->getId();
      std::vector<const SVFFunction *> &vec = indCalleeOf[rid];
      for (const SVFFunction *callee : kv.second)
        vec.push_back(callee);
      if (!vec.empty())
        ++nMapped;
    }
    std::cout << "  间接调用点 " << nCS << " 个,登记结果节点 " << nMapped
              << " 个" << std::endl;
  }

                                  
  bool isAllocaNode(const PAGNode *n) {
    if (!n || !n->hasValue())
      return false;
    const SVFValue *sv = n->getValue();
    if (!sv)
      return false;
    const Value *v = LLVMModuleSet::getLLVMModuleSet()->getLLVMValue(sv);
    return v && isa<AllocaInst>(v);
  }

                                                                 
                                             
                                           
                         
  void markFreedFields() {
    SaberCheckerAPI *api = SaberCheckerAPI::getCheckerAPI();
    unsigned nFree = 0;
    for (const CallICFGNode *cs : pag->getCallSiteSet()) {
      bool dealloc = api->isMemDealloc(cs);        
      if (!dealloc && ander->hasIndCSCallees(cs))                
        for (const SVFFunction *callee : ander->getIndCSCallees(cs))
          if (api->isMemDealloc(callee)) {
            dealloc = true;
            break;
          }
      if (!dealloc)
        continue;
      const auto &args = cs->getActualParms();
      if (args.empty())
        continue;
      ++nFree;
      traceFreedArgToField(args[0]->getId());                 
    }
    std::cout << "  free 族调用 " << nFree << " 处,登记被释放字段 "
              << freedFields.size() << " 个" << std::endl;
  }

                                                     
                                               
                
                                                                  
                                   
                                                      
                                              
                                                   
                                                   
                                            
                                                             
                                              
  void traceFreedArgToField(NodeID start) {
    std::set<NodeID> visited;
    std::queue<NodeID> wl;
    wl.push(start);
    visited.insert(start);
    int budget = 20000;
    while (!wl.empty() && budget-- > 0) {
      NodeID cur = wl.front();
      wl.pop();
      const PAGNode *n = pag->getGNode(cur);
      if (!n)
        continue;
      for (const PAGEdge *e : n->getInEdges()) {
        auto k = e->getEdgeKind();
        if (k == PAGEdge::Copy || k == PAGEdge::Ret || k == PAGEdge::Phi ||
            k == PAGEdge::Select || k == PAGEdge::Call) {
          NodeID s = e->getSrcID();
          if (visited.insert(s).second)
            wl.push(s);
        } else if (k == PAGEdge::Load) {
          NodeID q = e->getSrcID();
          auto it = nodeToField.find(q);                   
          if (it != nodeToField.end())
            freedFields.insert(it->second);
          const PAGNode *qn = pag->getGNode(q);                
          if (qn && isAllocaNode(qn))
            for (const PAGEdge *se : qn->getInEdges())
              if (se->getEdgeKind() == PAGEdge::Store) {
                NodeID sv = se->getSrcID();
                if (visited.insert(sv).second)
                  wl.push(sv);
              }
        }
      }
    }
  }

                                                      
                                                     
                                               
                                                           
  bool computeFieldOwnership(StructType *st, unsigned fld,
                             const std::vector<NodeID> &addrNodes) {
    // Rule 2(Destruction)/ Rule 3(Transfer)。
    if (freedFields.count({st, fld}))
      return true;
    // Rule 1(Creation)。
    for (NodeID a : addrNodes) {
      const PAGNode *an = pag->getGNode(a);
      if (!an)
        continue;
      for (const PAGEdge *e : an->getInEdges()) {
        if (e->getEdgeKind() == PAGEdge::Store &&
            flowsFromAllocation(e->getSrcID()))
          return true;
      }
    }
    return false;
  }

                                           
                                                          
                                                  
                                               
                                                  
  bool flowsFromAllocation(NodeID v) {
    std::set<NodeID> visited;
    std::queue<NodeID> wl;
    wl.push(v);
    visited.insert(v);
    int budget = 20000;         
    while (!wl.empty() && budget-- > 0) {
      NodeID cur = wl.front();
      wl.pop();

                                                    
                                                 
                                                    
      auto icIt = indCalleeOf.find(cur);
      if (icIt != indCalleeOf.end()) {
        for (const SVFFunction *callee : icIt->second) {
          if (SVFUtil::isHeapAllocExtFunViaRet(callee) ||
              SVFUtil::isReallocExtFun(callee))
            return true;
          if (!callee->isDeclaration() && pag->funHasRet(callee)) {
            NodeID r = pag->getFunRet(callee)->getId();
            if (visited.insert(r).second)
              wl.push(r);
          }
        }
      }

      const PAGNode *n = pag->getGNode(cur);
      if (!n)
        continue;
      for (const PAGEdge *e : n->getInEdges()) {
        auto k = e->getEdgeKind();
        if (k == PAGEdge::Addr) {
                                                      
          if (ander && ander->isHeapMemObj(e->getSrcID()))
            return true;
        } else if (k == PAGEdge::Copy || k == PAGEdge::Ret ||
                   k == PAGEdge::Phi || k == PAGEdge::Select) {
                                                 
                                                      
                                                       
          NodeID s = e->getSrcID();
          if (visited.insert(s).second)
            wl.push(s);
        } else if (k == PAGEdge::Load) {
                                                             
                                                 
                                        
          NodeID q = e->getSrcID();
          const PAGNode *qn = pag->getGNode(q);
          if (qn && isAllocaNode(qn))
            for (const PAGEdge *se : qn->getInEdges())
              if (se->getEdgeKind() == PAGEdge::Store) {
                NodeID sv = se->getSrcID();
                if (visited.insert(sv).second)
                  wl.push(sv);
              }
        }
      }
    }
    return false;
  }

  bool analyzeFieldMutability(StructType *structType, unsigned fieldIdx,
                              const std::vector<NodeID> &fieldNodes) {
    bool isMutable = false;

    for (NodeID fieldNodeId : fieldNodes) {
      if (hasModificationEvidence(fieldNodeId)) {
        isMutable = true;
        break;
      }
    }

    return isMutable;
  }

                                                  
                                        
                                                          
                                                        
                                                       
                                 
  bool hasModificationEvidence(NodeID nodeId) {
    const PAGNode *node = pag->getGNode(nodeId);
    if (!node)
      return false;

                                             
                                                                
    for (const PAGEdge *edge : node->getOutEdges()) {
      if (edge->getEdgeKind() == PAGEdge::Load) {
        NodeID loadedNodeId = edge->getDstID();
        if (hasModificationAfterLoad(loadedNodeId))
          return true;
      }
    }

                                                   
    if (isPassedToModifyingFunction(nodeId))
      return true;

    return false;
  }

                                        
                                                 
                                        
           
                                                        
                                                     
                                             
                                                        
                                    
  bool hasModificationAfterLoad(NodeID loadedNodeId) {
    const PAGNode *loadedNode = pag->getGNode(loadedNodeId);
    if (!loadedNode)
      return false;

    std::set<NodeID> visited;
    std::queue<NodeID> worklist;

    worklist.push(loadedNodeId);
    visited.insert(loadedNodeId);
    int budget = 20000;                                   

    while (!worklist.empty() && budget-- > 0) {
      NodeID currentId = worklist.front();
      worklist.pop();

      const PAGNode *currentNode = pag->getGNode(currentId);
      if (!currentNode)
        continue;

                                              
      for (const PAGEdge *edge : currentNode->getInEdges()) {
        if (edge->getEdgeKind() == PAGEdge::Store)
          return true;
      }

                   
      for (const PAGEdge *edge : currentNode->getOutEdges()) {
        auto kind = edge->getEdgeKind();
        if (kind == PAGEdge::Copy || kind == PAGEdge::Gep ||
            kind == PAGEdge::Call) {
                                                          
          NodeID dstId = edge->getDstID();
          if (visited.insert(dstId).second)
            worklist.push(dstId);
        } else if (kind == PAGEdge::Store) {
                                                      
                                        
          const PAGNode *locNode = pag->getGNode(edge->getDstID());
          if (locNode && isAllocaNode(locNode))
            for (const PAGEdge *le : locNode->getOutEdges())
              if (le->getEdgeKind() == PAGEdge::Load)
                if (visited.insert(le->getDstID()).second)
                  worklist.push(le->getDstID());
        }
      }
    }

    return false;
  }

                           
  bool isPassedToModifyingFunction(NodeID nodeId) {
                         
    if (cachedData.modifyingCallArgNodes.empty()) {
      return false;
    }

                         
    std::set<NodeID> reachable;
    std::queue<NodeID> worklist;

    worklist.push(nodeId);
    reachable.insert(nodeId);

    while (!worklist.empty()) {
      NodeID currentID = worklist.front();
      worklist.pop();

                                
      if (cachedData.modifyingCallArgNodes.count(currentID)) {
        return true;
      }

      const PAGNode *node = pag->getGNode(currentID);
      if (!node)
        continue;

      for (const PAGEdge *edge : node->getOutEdges()) {
        NodeID dstID = edge->getDstID();

        if (const LoadStmt *load = SVFUtil::dyn_cast<LoadStmt>(edge)) {
          if (reachable.find(dstID) == reachable.end()) {
            worklist.push(dstID);
            reachable.insert(dstID);
          }
        } else if (const CopyStmt *copy = SVFUtil::dyn_cast<CopyStmt>(edge)) {
          if (reachable.find(dstID) == reachable.end()) {
            worklist.push(dstID);
            reachable.insert(dstID);
          }
        } else if (const GepStmt *gep = SVFUtil::dyn_cast<GepStmt>(edge)) {
          if (reachable.find(dstID) == reachable.end()) {
            worklist.push(dstID);
            reachable.insert(dstID);
          }
        }
      }
    }

    return false;
  }

                  
                                               
  bool isFunctionPointerType(Type *type) {
    if (!type)
      return false;

                
    if (!type->isPointerTy())
      return false;

               
    Type *elemType = type->getPointerElementType();
    if (!elemType)
      return false;

                    
    return elemType->isFunctionTy();
  }

                      
  bool isCallToModifyingFunction(const std::string &callStr) {
    if (callStr.find("call") == std::string::npos) {
      return false;
    }

    static const std::vector<std::string> modifyingFunctionNames = {
        "@strcpy",   "@strcat", "@strncpy", "@strncat", "@sprintf",
        "@snprintf", "@memset", "@memcpy",  "@memmove", "@scanf",
        "@gets",     "@fgets",  "@strtok",  "@strtok_r"};

    for (const auto &funcName : modifyingFunctionNames) {
      if (callStr.find(funcName) != std::string::npos) {
        return true;
      }
    }

    return false;
  }

                     
  NodeInfo getNodeInfo(const PAGNode *node) {
    if (!node)
      return NodeInfo();

    NodeID nodeId = node->getId();

           
    auto it = cachedData.nodeInfoCache.find(nodeId);
    if (it != cachedData.nodeInfoCache.end()) {
      return it->second;
    }

            
    NodeInfo nodeInfo = computeNodeInfo(node);
    cachedData.nodeInfoCache[nodeId] = nodeInfo;
    return nodeInfo;
  }

             
  NodeInfo computeNodeInfo(const PAGNode *node) {
    NodeInfo nodeInfo;
    nodeInfo.id = node->getId();
    nodeInfo.description = node->toString();
    nodeInfo.hasValue = node->hasValue();
    nodeInfo.file_name = "";
    nodeInfo.line = -1;

    std::string desc = nodeInfo.description;
    size_t lnPos = desc.find("\"ln\": ");
    size_t flPos = desc.find("\"fl\": ");
    size_t filePos = desc.find("\"file\": ");

    if (lnPos != std::string::npos) {
      lnPos += 6;
      size_t commaPos = desc.find(",", lnPos);
      size_t bracePos = desc.find("}", lnPos);
      size_t endPos =
          std::min(commaPos != std::string::npos ? commaPos : desc.length(),
                   bracePos != std::string::npos ? bracePos : desc.length());

      try {
        nodeInfo.line = std::stoi(desc.substr(lnPos, endPos - lnPos));
      } catch (...) {
      }
    }

    if (flPos != std::string::npos) {
      flPos += 6;
      if (flPos < desc.length() && desc[flPos] == '\"') {
        flPos++;
        size_t endQuote = desc.find('\"', flPos);
        if (endQuote != std::string::npos) {
          nodeInfo.file_name = desc.substr(flPos, endQuote - flPos);
        }
      }
    } else if (filePos != std::string::npos) {
      filePos += 8;
      if (filePos < desc.length() && desc[filePos] == '\"') {
        filePos++;
        size_t endQuote = desc.find('\"', filePos);
        if (endQuote != std::string::npos) {
          nodeInfo.file_name = desc.substr(filePos, endQuote - filePos);
        }
      }
    }

    if (node->hasValue()) {
      const SVFValue *val = node->getValue();
      if (val) {
        try {
          const Value *llvmVal =
              LLVMModuleSet::getLLVMModuleSet()->getLLVMValue(val);
          if (llvmVal) {
            nodeInfo.valueName = llvmVal->getName().str();
          }
        } catch (...) {
          nodeInfo.valueName = "获取LLVM值时出错";
        }
      }
    }

    return nodeInfo;
  }

                     
  std::string getTypeString(Type *type) {
    if (!type)
      return "null";

    if (type->isIntegerTy()) {
      return "i" + std::to_string(type->getIntegerBitWidth());
    } else if (type->isFloatTy()) {
      return "float";
    } else if (type->isDoubleTy()) {
      return "double";
    } else if (type->isPointerTy()) {
      Type *elemType = type->getPointerElementType();
      return getTypeString(elemType) + "*";
    } else if (type->isArrayTy()) {
      Type *elemType = type->getArrayElementType();
      uint64_t numElements = type->getArrayNumElements();
      return "[" + std::to_string(numElements) + " x " +
             getTypeString(elemType) + "]";
    } else if (type->isStructTy()) {
      StructType *structType = dyn_cast<StructType>(type);
      if (structType->hasName()) {
        return structType->getName().str();
      } else {
        return "anonymous_struct";
      }
    } else if (type->isFunctionTy()) {
      return "function";
    } else if (type->isVoidTy()) {
      return "void";
    } else {
      return "unknown_type";
    }
  }

  void generateAnalysisReport(const std::string &outputFile) {
    nlohmann::json reportJson;

    for (const auto &structPair : structInfoMap) {
      StructType *structType = structPair.first;
      const StructInfo &structInfo = structPair.second;

      nlohmann::json structJson;

      nlohmann::json fieldsArrayJson = nlohmann::json::array();

      for (const auto &fieldTypePair : structInfo.fieldTypes) {
        unsigned fieldIdx = fieldTypePair.first;
        const std::string &fieldType = fieldTypePair.second;

        nlohmann::json fieldJson;
        fieldJson["field_index"] = fieldIdx;
        fieldJson["field_type"] = fieldType;
        fieldJson["is_pointer"] = (structInfo.pointerFields.find(fieldIdx) !=
                                   structInfo.pointerFields.end());

        auto fieldNameIt = structInfo.fieldNames.find(fieldIdx);
        if (fieldNameIt != structInfo.fieldNames.end()) {
          fieldJson["field_name"] = fieldNameIt->second;
        } else {
          fieldJson["field_name"] = "field_" + std::to_string(fieldIdx);
        }

        if (structInfo.pointerFields.find(fieldIdx) !=
            structInfo.pointerFields.end()) {
          auto nullabilityIt = structInfo.fieldNullability.find(fieldIdx);
          if (nullabilityIt != structInfo.fieldNullability.end()) {
            fieldJson["nullability"] =
                nullabilityIt->second ? "Nullable" : "Not_nullable";
          } else {
            fieldJson["nullability"] = "Unknown";
          }

          auto ownershipIt = structInfo.fieldOwnership.find(fieldIdx);
          if (ownershipIt != structInfo.fieldOwnership.end()) {
            fieldJson["ownership"] = ownershipIt->second;
          } else {
            fieldJson["ownership"] = "Unknown";
          }

          auto mutableIt = structInfo.fieldMutable.find(fieldIdx);
          if (mutableIt != structInfo.fieldMutable.end()) {
            fieldJson["mutability"] =
                mutableIt->second ? "Mutable" : "Immutable";
          } else {
            fieldJson["mutability"] = "Unknown";
          }
        }

        fieldsArrayJson.push_back(fieldJson);
      }
      structJson["fields"] = fieldsArrayJson;

      nlohmann::json usagePathsJson;
      for (const auto &fieldUsageGroupPair : structInfo.fieldUsageNodeGroups) {
        unsigned fieldIdx = fieldUsageGroupPair.first;
        const std::map<NodeID, std::vector<std::vector<NodeID>>>
            &fieldNodeGroups = fieldUsageGroupPair.second;

        std::string fieldKey = "field_" + std::to_string(fieldIdx);
        nlohmann::json fieldPathsArrayJson = nlohmann::json::array();

        for (const auto &groupPair : fieldNodeGroups) {
          NodeID fieldNodeId = groupPair.first;
          const std::vector<std::vector<NodeID>> &pathsList = groupPair.second;

          nlohmann::json fieldNodeGroupJson;
          fieldNodeGroupJson["field_node_id"] = fieldNodeId;

          const PAGNode *fieldNode = pag->getGNode(fieldNodeId);
          if (fieldNode) {
            NodeInfo fieldNodeInfo = getNodeInfo(fieldNode);
            std::string fieldLocation;
            if (!fieldNodeInfo.file_name.empty() && fieldNodeInfo.line > 0) {
              fieldLocation = fieldNodeInfo.file_name + ":" +
                              std::to_string(fieldNodeInfo.line);
              fieldNodeGroupJson["field_node_location"] = fieldLocation;
            }
          }

          nlohmann::json pathsArrayJson = nlohmann::json::array();

          for (size_t pathIdx = 0; pathIdx < pathsList.size(); pathIdx++) {
            const std::vector<NodeID> &singlePath = pathsList[pathIdx];

            if (!singlePath.empty()) {
              std::vector<std::string> locationPath;

              for (NodeID pathNodeId : singlePath) {
                const PAGNode *node = pag->getGNode(pathNodeId);
                if (!node)
                  continue;

                NodeInfo nodeInfo = getNodeInfo(node);

                std::string location;
                if (!nodeInfo.file_name.empty() && nodeInfo.line > 0) {
                  location =
                      nodeInfo.file_name + ":" + std::to_string(nodeInfo.line);
                  locationPath.push_back(location);
                }
              }

              std::vector<std::string> deduplicatedPath;
              std::string lastLocation = "";

              for (const std::string &location : locationPath) {
                if (location != lastLocation) {
                  deduplicatedPath.push_back(location);
                  lastLocation = location;
                }
              }

              nlohmann::json singlePathJson = nlohmann::json::array();
              for (const std::string &location : deduplicatedPath) {
                singlePathJson.push_back(location);
              }

              if (!singlePathJson.empty()) {
                pathsArrayJson.push_back(singlePathJson);
              }
            }
          }

          fieldNodeGroupJson["paths"] = pathsArrayJson;
          fieldNodeGroupJson["path_count"] = pathsArrayJson.size();

          if (!pathsArrayJson.empty()) {
            fieldPathsArrayJson.push_back(fieldNodeGroupJson);
          }
        }

        if (!fieldPathsArrayJson.empty()) {
          usagePathsJson[fieldKey] = fieldPathsArrayJson;
        }
      }

      structJson["usage_paths"] = usagePathsJson;
      reportJson[structInfo.name] = structJson;
    }

    std::ofstream outFile(outputFile);
    if (!outFile.is_open()) {
      std::cerr << "无法打开输出文件: " << outputFile << std::endl;
      return;
    }

    outFile << reportJson.dump(4) << std::endl;
    outFile.close();

    std::cout << "结构体分析报告已保存到: " << outputFile << std::endl;
  }
};

int main(int argc, char **argv) {
  std::cout << "SVF 结构体分析开始" << std::endl;

  std::vector<std::string> irFiles;
  for (int i = 1; i < argc; ++i) {
    irFiles.emplace_back(argv[i]);
  }

                  
  for (const std::string &irFile : irFiles) {
    std::ifstream inFile(irFile);
    std::vector<std::string> lines;
    std::string line;
    while (std::getline(inFile, line)) {
      lines.push_back(line);
    }
    inFile.close();
    irContents[irFile] = lines;
  }

              
  std::cout << "分析以下IR文件:" << std::endl;
  for (size_t i = 0; i < irFiles.size(); i++) {
    std::cout << "[" << i << "]: " << irFiles[i] << std::endl;
  }

           
  std::cout << "初始化SVF..." << std::endl;
  SVFModule *svfModule =
      LLVMModuleSet::getLLVMModuleSet()->buildSVFModule(irFiles);

            
  std::cout << "构建SVFIR..." << std::endl;
  SVFIRBuilder builder(svfModule);
  SVFIR *pag = builder.build();

  /// Create struct analyzer
  StructAnalyzer *analyzer = new StructAnalyzer(pag);

  /// Whole-program Andersen points-to (SVF). Used to (a) resolve indirect
  /// calls — so function-pointer allocators / deallocators (libcsv
  /// realloc_func, bzip2 bzalloc/bzfree) are handled like direct ones —
  /// and (b) identify heap objects. This replaces the old ContextDDA
  /// (demand-driven, never actually used, and the O0 timeout culprit).
  /// Andersen is whole-program but near-linear in practice; the struct
  /// analysis itself is single-pass PAG indexing, so O0 input is fine.
  std::cout << "执行 Andersen 指针分析..." << std::endl;
  AndersenWaveDiff *ander = AndersenWaveDiff::createAndersenWaveDiff(pag);

  /// Analyze structs with pointer fields (PtrTrans ownership rules, on the
  /// SVFIR + Andersen — no IR text scanning; runs on -O0 IR where every
  /// struct field access is a regular `getelementptr %struct.S ... i32 N`).
  std::cout << "开始结构体分析..." << std::endl;
  analyzer->start_analyzeStructsWithPointerFields(ander);

                     
  std::string outputFile = "struct_analysis_report.json";
  analyzer->generateAnalysisReport(outputFile);

  /// Cleanup
  delete analyzer;
  SVFIR::releaseSVFIR();
  LLVMModuleSet::releaseLLVMModuleSet();

  std::cout << "结构体分析完成." << std::endl;
  return 0;
}
