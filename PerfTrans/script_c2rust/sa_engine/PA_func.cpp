// ===========================================================================
                                                        
// ===========================================================================
                        
             
              
                                             
                        
//
                                                                
                                            
                                         
//
                         
            
                                                   
                                                      
                                                       
           
                                                          
                                                   
                                
//
                                                            
                                                
//
                                                     
                                                
//
                                                 
// ===========================================================================
#include "SABER/SaberCheckerAPI.h"
#include "Util/SVFUtil.h"
#include "WPA/Andersen.h"
#include "MemoryModel/PointerAnalysisImpl.h"
#include "SVF-LLVM/LLVMUtil.h"
#include "SVF-LLVM/SVFIRBuilder.h"
#include "SVFIR/SVFFileSystem.h"
#include "Util/Options.h"
#include <algorithm>
#include <chrono>
#include <ctime>
#include <deque>
#include <fstream>
#include <iostream>
#include <map>
#include <nlohmann/json.hpp>
#include <queue>
#include <set>
#include <string>
#include <vector>

using namespace llvm;
using namespace std;
using namespace SVF;

class FunctionParamAnalyzer {
private:
  SVFIR *pag;
  AndersenWaveDiff *ander;
  PTACallGraph *callgraph;

                                                         
  struct CallsiteInfo {
    std::string callStmt;                           
    NodeID callerArgId = 0;                             
                                                         
                                                          
                                                          
                                                       
    bool sameObject = false;
    std::string pointsToType; // struct / function / array / primitive / pointer
    std::string pointsTo;                  
  };

  struct ParamInfo {
    NodeID paramID;
    std::string funcName;
    std::string paramName;
    std::string ownership = "Borrowed";   // Owning | Borrowed
    std::string mutability = "Immutable"; // Mutable | Immutable
    std::string nullability = "Nullable";                           
    std::string countResult = "Unknown";  // Scalar | Array | Unknown
    std::string structTypeName;                           
    std::set<int> structFields;                         
    bool structVariantIndex = false;                      
    std::vector<CallsiteInfo> callsites;
  };

  struct ReturnInfo {
    NodeID returnID = (NodeID)-1;
    std::string funcName;
    std::string returnName = "return_value";
    std::string ownership = "Borrowed";
    std::string mutability = "Immutable";
    std::string nullability = "Nullable";
    std::string lifeResult = "No_Depends";
  };

                                                         
  std::map<const SVFFunction *, std::map<NodeID, ParamInfo>> funcParams;
  std::map<const SVFFunction *, ReturnInfo> funcReturns;
  std::map<const SVFFunction *, int> funcLevel;                  

                                                                  
                                                
  std::set<NodeID> freedActualArgs;
                                                     
  std::set<NodeID> modifyingCallArgNodes;
                                                        
                                         
  std::map<NodeID, std::vector<const SVFFunction *>> indCalleeOf;
                                               
  std::map<const SVFFunction *, std::vector<const CallICFGNode *>>
      callsitesByFunc;
                                                 
                                           
  std::map<const SVFFunction *, std::set<NodeID>> scopeWrittenObjs;

public:
  FunctionParamAnalyzer(SVFIR *p, AndersenWaveDiff *a) : pag(p), ander(a) {
    callgraph = ander->getPTACallGraph();
  }

                                                      
  void run() {
    std::cout << "构建预计算索引..." << std::endl;
    buildIndirectCalleeMap();
    buildFreeIndex();
    buildModifyingCallIndex();
    for (const CallICFGNode *cs : pag->getCallSiteSet())
      if (const SVFFunction *caller = cs->getCaller())
        callsitesByFunc[caller].push_back(cs);

    std::cout << "构建调用图层级..." << std::endl;
    buildCallGraphLevels();
    buildWrittenObjsIndex();

    int maxLevel = 0;
    for (const auto &kv : funcLevel)
      maxLevel = std::max(maxLevel, kv.second);

                                                    
    for (int level = 0; level <= maxLevel; ++level) {
      for (const auto &kv : funcLevel) {
        if (kv.second != level)
          continue;
        const SVFFunction *func = kv.first;
        if (func->isDeclaration())
          continue;
        analyzeFunction(func);
      }
    }
  }

  // =========================================================================
          
  // =========================================================================

                                              
  void buildIndirectCalleeMap() {
    for (const auto &kv : ander->getIndCallMap()) {
      const CallICFGNode *cs = kv.first;
      const RetICFGNode *ret = cs->getRetICFGNode();
      if (!ret || !ret->getActualRet())
        continue;
      NodeID rid = ret->getActualRet()->getId();
      std::vector<const SVFFunction *> &vec = indCalleeOf[rid];
      for (const SVFFunction *callee : kv.second)
        vec.push_back(callee);
    }
  }

                                                 
                                          
  void buildFreeIndex() {
    SaberCheckerAPI *api = SaberCheckerAPI::getCheckerAPI();
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
      if (!args.empty())
        freedActualArgs.insert(args[0]->getId());
    }
    std::cout << "  free 族释放实参 " << freedActualArgs.size() << " 个"
              << std::endl;
  }

                                                      
                                          
                                                             
                                                                       
                                                
  void buildModifyingCallIndex() {
    static const std::vector<std::string> kModifying = {
        "memcpy", "memmove", "memset",  "strcpy",   "strncpy",  "strcat",
        "strncat", "sprintf", "snprintf", "vsprintf", "vsnprintf"};
    for (const CallICFGNode *cs : pag->getCallSiteSet()) {
      const SVFFunction *callee = SVFUtil::getCallee(cs->getCallSite());
      if (!callee)
        continue;
      const std::string name = callee->getName();
      bool modifying = false;
      for (const std::string &m : kModifying)
        if (name.find(m) != std::string::npos) {
          modifying = true;
          break;
        }
      if (!modifying)
        continue;
      const auto &args = cs->getActualParms();
      if (!args.empty())                  
        modifyingCallArgNodes.insert(args[0]->getId());
    }
    std::cout << "  修改型函数目的实参 " << modifyingCallArgNodes.size()
              << " 个" << std::endl;
  }

                                                 
                                                    
                                         
  void buildCallGraphLevels() {
    std::map<const SVFFunction *, std::set<const SVFFunction *>> callees;
    for (auto it = callgraph->begin(), eit = callgraph->end(); it != eit;
         ++it) {
      const SVFFunction *f = it->second->getFunction();
      if (!f || f->isDeclaration())
        continue;
      funcLevel[f] = 0;
      for (auto e = it->second->OutEdgeBegin(), ee = it->second->OutEdgeEnd();
           e != ee; ++e) {
        const SVFFunction *g = (*e)->getDstNode()->getFunction();
        if (g && !g->isDeclaration() && g != f)
          callees[f].insert(g);
      }
    }
    for (size_t i = 0; i <= funcLevel.size(); ++i) {
      bool changed = false;
      for (const auto &kv : callees)
        for (const SVFFunction *g : kv.second) {
          int want = funcLevel[g] + 1;
          if (funcLevel[kv.first] < want) {
            funcLevel[kv.first] = want;
            changed = true;
          }
        }
      if (!changed)
        break;
    }
  }

                                                   
                                               
                                
  NodeID gepCopyRoot(NodeID v) {
    std::set<NodeID> seen{v};
    int budget = 2000;
    while (budget-- > 0) {
      const PAGNode *n = pag->getGNode(v);
      if (!n)
        break;
      NodeID next = v;
      for (const PAGEdge *e : n->getInEdges()) {
        auto k = e->getEdgeKind();
        if (k == PAGEdge::Gep || k == PAGEdge::Copy) {
          next = e->getSrcID();
          break;
        }
      }
      if (next == v || !seen.insert(next).second)
        break;
      v = next;
    }
    return v;
  }

                                                      
                                                      
                                                   
                                          
                         
  void buildWrittenObjsIndex() {
                                                      
                                                    
    std::map<const SVFFunction *, std::set<NodeID>> ownWrites;
    ICFG *icfg = pag->getICFG();
    for (auto it = icfg->begin(), eit = icfg->end(); it != eit; ++it) {
      const SVFFunction *f = it->second->getFun();
      if (!f)
        continue;
      for (const SVFStmt *stmt : pag->getSVFStmtList(it->second)) {
        const StoreStmt *st = SVFUtil::dyn_cast<StoreStmt>(stmt);
        if (!st)
          continue;
        NodeID root = gepCopyRoot(st->getLHSVarID());
        for (NodeID o : ander->getPts(root))
          ownWrites[f].insert(o);
      }
    }
                      
    std::map<const SVFFunction *, std::set<const SVFFunction *>> callees;
    for (auto it = callgraph->begin(), eit = callgraph->end(); it != eit;
         ++it) {
      const SVFFunction *f = it->second->getFunction();
      if (!f)
        continue;
      for (auto e = it->second->OutEdgeBegin(), ee = it->second->OutEdgeEnd();
           e != ee; ++e) {
        const SVFFunction *g = (*e)->getDstNode()->getFunction();
        if (g && g != f)
          callees[f].insert(g);
      }
    }
                                                            
                                       
    std::vector<const SVFFunction *> order;
    for (const auto &kv : funcLevel)
      order.push_back(kv.first);
    std::sort(order.begin(), order.end(),
              [this](const SVFFunction *a, const SVFFunction *b) {
                return funcLevel.at(a) < funcLevel.at(b);
              });
    for (const SVFFunction *f : order) {
      std::set<NodeID> &dst = scopeWrittenObjs[f];
      auto ow = ownWrites.find(f);
      if (ow != ownWrites.end())
        dst.insert(ow->second.begin(), ow->second.end());
      for (const SVFFunction *g : callees[f]) {
        auto git = scopeWrittenObjs.find(g);
        if (git != scopeWrittenObjs.end())
          dst.insert(git->second.begin(), git->second.end());
      }
    }
  }

  // =========================================================================
                
  // =========================================================================

                                   
  bool isAllocaNode(const PAGNode *n) {
    if (!n || !n->hasValue())
      return false;
    const SVFValue *sv = n->getValue();
    if (!sv)
      return false;
    const Value *v = LLVMModuleSet::getLLVMModuleSet()->getLLVMValue(sv);
    return v && isa<AllocaInst>(v);
  }

                                                   
                                                     
                                                       
                                                        
                                    
  std::set<NodeID> collectValueSet(NodeID seed) {
    std::set<NodeID> S{seed};
    std::queue<NodeID> wl;
    wl.push(seed);
    int budget = 20000;
    while (!wl.empty() && budget-- > 0) {
      NodeID cur = wl.front();
      wl.pop();
      const PAGNode *n = pag->getGNode(cur);
      if (!n)
        continue;
      for (const PAGEdge *e : n->getOutEdges()) {
        auto k = e->getEdgeKind();
        if (k == PAGEdge::Copy) {
          if (S.insert(e->getDstID()).second)
            wl.push(e->getDstID());
        } else if (k == PAGEdge::Store) {
                                                       
                                          
          const PAGNode *ln = pag->getGNode(e->getDstID());
          if (ln && isAllocaNode(ln))
            for (const PAGEdge *le : ln->getOutEdges())
              if (le->getEdgeKind() == PAGEdge::Load)
                if (S.insert(le->getDstID()).second)
                  wl.push(le->getDstID());
        }
      }
    }
    return S;
  }

                                           
                                                            
                                                   
                                           
                                             
                                         
  bool flowsFromAllocation(NodeID v) {
    std::set<NodeID> visited{v};
    std::queue<NodeID> wl;
    wl.push(v);
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
          if (visited.insert(e->getSrcID()).second)
            wl.push(e->getSrcID());
        } else if (k == PAGEdge::Load) {
          NodeID q = e->getSrcID();
          const PAGNode *qn = pag->getGNode(q);
          if (qn && isAllocaNode(qn))
            for (const PAGEdge *se : qn->getInEdges())
              if (se->getEdgeKind() == PAGEdge::Store)
                if (visited.insert(se->getSrcID()).second)
                  wl.push(se->getSrcID());
        }
      }
    }
    return false;
  }

                                                   
                                              
  std::set<NodeID> derefClosure(const std::set<NodeID> &valueSet) {
    std::set<NodeID> S = valueSet;
    std::queue<NodeID> wl;
    for (NodeID v : valueSet)
      wl.push(v);
    int budget = 20000;
    while (!wl.empty() && budget-- > 0) {
      NodeID cur = wl.front();
      wl.pop();
      const PAGNode *n = pag->getGNode(cur);
      if (!n)
        continue;
      for (const PAGEdge *e : n->getOutEdges()) {
        auto k = e->getEdgeKind();
        if (k == PAGEdge::Copy || k == PAGEdge::Gep)
          if (S.insert(e->getDstID()).second)
            wl.push(e->getDstID());
      }
    }
    return S;
  }

                                                      
                                 
  bool pointeeMutated(const std::set<NodeID> &valueSet) {
    for (NodeID v : valueSet)
      if (modifyingCallArgNodes.count(v))
        return true;
    std::set<NodeID> closure = derefClosure(valueSet);
    for (NodeID v : closure) {
                                              
                                             
      const PAGNode *n = pag->getGNode(v);
      if (!n)
        continue;
      for (const PAGEdge *e : n->getInEdges())
        if (e->getEdgeKind() == PAGEdge::Store)
          return true;
    }
    return false;
  }

                          
  bool isGlobalValueNode(const PAGNode *n) {
    if (!n || !n->hasValue())
      return false;
    const SVFValue *sv = n->getValue();
    if (!sv)
      return false;
    const Value *v = LLVMModuleSet::getLLVMModuleSet()->getLLVMValue(sv);
    return v && isa<GlobalVariable>(v);
  }

  // =========================================================================
          
  // =========================================================================
  void analyzeFunction(const SVFFunction *func) {
    std::cout << "分析函数 " << func->getName()
              << " (level " << funcLevel[func] << ")" << std::endl;

                                                               
    std::map<NodeID, ParamInfo> &params = funcParams[func];
    for (unsigned i = 0; i < func->arg_size(); ++i) {
      const SVFArgument *arg = func->getArg(i);
      if (!arg->getType()->isPointerTy())
        continue;
      NodeID pid = pag->getValueNode(arg);
      ParamInfo info;
      info.paramID = pid;
      info.funcName = func->getName();
      info.paramName = arg->getName();
      info.countResult = analyzeParamCount(func, i);
      classifyPointee(func, i, info);                                 
      params[pid] = info;
    }

                                                   
    for (auto &kv : params) {
      NodeID pid = kv.first;
      ParamInfo &info = kv.second;
      std::set<NodeID> valueSet = collectValueSet(pid);

      info.ownership = computeParamOwnership(func, pid, valueSet) ? "Owning"
                                                                  : "Borrowed";
      info.mutability =
          computeParamMutability(func, pid, valueSet,
                                 !info.structTypeName.empty())
              ? "Mutable"
              : "Immutable";
                                        
      info.nullability = "Nullable";
      analyzeStructMemberUsage(func, valueSet, info);
    }

                                                        
    collectCallsites(func, params);

                                                                
    analyzeReturn(func);
  }

                                                                    
                                         
                                                              
                                       
  std::string analyzeParamCount(const SVFFunction *func, unsigned argIdx) {
    LLVMModuleSet *modSet = LLVMModuleSet::getLLVMModuleSet();
    const Value *llvmF = modSet->getLLVMValue(func);
    const Function *F = dyn_cast_or_null<Function>(llvmF);
    if (!F || argIdx >= F->arg_size())
      return "Unknown";
    const Argument *argLLVM = F->getArg(argIdx);

    std::set<const Value *> tracked{argLLVM};
    std::deque<const Value *> worklist{argLLVM};
    while (!worklist.empty()) {
      const Value *v = worklist.front();
      worklist.pop_front();
      for (const User *U : v->users()) {
        const StoreInst *st = dyn_cast<StoreInst>(U);
        if (!st || st->getValueOperand() != v)
          continue;
        const Value *dst = st->getPointerOperand();
        if (!isa<AllocaInst>(dst))
          continue;
        for (const User *DU : dst->users()) {
          const LoadInst *ld = dyn_cast<LoadInst>(DU);
          if (!ld || ld->getPointerOperand() != dst)
            continue;
          if (tracked.insert(ld).second)
            worklist.push_back(ld);
        }
      }
    }
    for (const Value *v : tracked) {
      for (const User *U : v->users()) {
        const GetElementPtrInst *gep = dyn_cast<GetElementPtrInst>(U);
        if (!gep || gep->getPointerOperand() != v)
          continue;
        if (gep->getNumIndices() == 0)
          continue;
        const Value *firstIdx = gep->getOperand(1);
        if (const ConstantInt *ci = dyn_cast<ConstantInt>(firstIdx)) {
          if (!ci->isZero())
            return "Array";
        } else {
          return "Array";
        }
      }
    }
    return "Scalar";
  }

                                                        
  void classifyPointee(const SVFFunction *func, unsigned argIdx,
                       ParamInfo &info) {
    LLVMModuleSet *modSet = LLVMModuleSet::getLLVMModuleSet();
    const Function *F = dyn_cast_or_null<Function>(modSet->getLLVMValue(func));
    if (!F || argIdx >= F->arg_size())
      return;
    Type *t = F->getArg(argIdx)->getType();
    if (!t->isPointerTy())
      return;
    Type *pointee = t->getPointerElementType();
    if (!pointee)
      return;
    if (pointee->isStructTy()) {
      StructType *st = dyn_cast<StructType>(pointee);
      if (st && st->hasName()) {
        std::string n = st->getName().str();
        info.structTypeName =
            (n.rfind("struct.", 0) == 0) ? n.substr(7) : n;
      } else {
        info.structTypeName = "anonymous_struct";
      }
    }
  }

                                                      
  std::pair<std::string, std::string> pointeeStrings(const SVFFunction *func,
                                                     unsigned argIdx) {
    LLVMModuleSet *modSet = LLVMModuleSet::getLLVMModuleSet();
    const Function *F = dyn_cast_or_null<Function>(modSet->getLLVMValue(func));
    if (!F || argIdx >= F->arg_size())
      return {"pointer", "unknown"};
    Type *t = F->getArg(argIdx)->getType();
    if (!t->isPointerTy())
      return {"pointer", "unknown"};
    Type *pointee = t->getPointerElementType();
    if (pointee->isStructTy()) {
      StructType *st = dyn_cast<StructType>(pointee);
      if (st && st->hasName()) {
        std::string n = st->getName().str();
        return {"struct", n.rfind("struct.", 0) == 0 ? n.substr(7) : n};
      }
      return {"struct", "anonymous_struct"};
    }
    if (pointee->isFunctionTy())
      return {"function", "function"};
    if (pointee->isArrayTy())
      return {"array", typeString(pointee)};
    if (pointee->isPointerTy())
      return {"pointer", typeString(pointee)};
    return {"primitive", typeString(pointee)};
  }

  std::string typeString(Type *type) {
    if (!type)
      return "void";
    if (type->isIntegerTy())
      return "i" + std::to_string(type->getIntegerBitWidth());
    if (type->isFloatTy())
      return "float";
    if (type->isDoubleTy())
      return "double";
    if (type->isVoidTy())
      return "void";
    if (type->isPointerTy())
      return typeString(type->getPointerElementType()) + "*";
    if (type->isArrayTy())
      return "[" + std::to_string(type->getArrayNumElements()) + " x " +
             typeString(type->getArrayElementType()) + "]";
    if (type->isStructTy()) {
      StructType *st = dyn_cast<StructType>(type);
      return st && st->hasName() ? st->getName().str() : "anonymous_struct";
    }
    if (type->isFunctionTy())
      return "function";
    return "unknown";
  }

  // =========================================================================
                                  
  // =========================================================================
  bool computeParamOwnership(const SVFFunction *func, NodeID paramID,
                             const std::set<NodeID> &valueSet) {
                                                   
    for (NodeID v : valueSet)
      if (freedActualArgs.count(v))
        return true;

                                   
    if (storedToPersistentLocation(func, paramID, valueSet))
      return true;

                                                    
                                      
    for (const CallICFGNode *cs : callsitesByFunc[func]) {
      const SVFFunction *callee = SVFUtil::getCallee(cs->getCallSite());
      if (!callee || callee->isDeclaration())
        continue;
      auto cit = funcParams.find(callee);
      if (cit == funcParams.end())
        continue;
      for (const SVFStmt *stmt : pag->getSVFStmtList(cs)) {
        const CallPE *callPE = SVFUtil::dyn_cast<CallPE>(stmt);
        if (!callPE)
          continue;
        if (!valueSet.count(callPE->getRHSVarID()))             
          continue;
        auto pit = cit->second.find(callPE->getLHSVarID());             
        if (pit != cit->second.end() && pit->second.ownership == "Owning")
          return true;
      }
    }
    return false;
  }

                                                   
                                           
                                 
  bool storedToPersistentLocation(const SVFFunction *func, NodeID paramID,
                                  const std::set<NodeID> &valueSet) {
    std::set<NodeID> otherParams;
    for (unsigned i = 0; i < func->arg_size(); ++i) {
      const SVFArgument *a = func->getArg(i);
      if (a->getType()->isPointerTy()) {
        NodeID id = pag->getValueNode(a);
        if (id != paramID)
          otherParams.insert(id);
      }
    }
    for (NodeID v : valueSet) {
      const PAGNode *n = pag->getGNode(v);
      if (!n)
        continue;
      for (const PAGEdge *e : n->getOutEdges()) {
        if (e->getEdgeKind() != PAGEdge::Store)
          continue;
        NodeID loc = e->getDstID();
                                               
                                    
        std::set<NodeID> visited{loc};
        std::queue<std::pair<NodeID, bool>> wl;                     
        wl.push({loc, false});
        int budget = 4000;
        while (!wl.empty() && budget-- > 0) {
          auto [cur, sawGep] = wl.front();
          wl.pop();
          const PAGNode *cn = pag->getGNode(cur);
          if (!cn)
            continue;
          if (isGlobalValueNode(cn))
            return true;
          if (sawGep && otherParams.count(cur))
            return true;
          for (const PAGEdge *ie : cn->getInEdges()) {
            auto k = ie->getEdgeKind();
            if (k == PAGEdge::Gep || k == PAGEdge::Copy ||
                k == PAGEdge::Load) {
              bool g = sawGep || (k == PAGEdge::Gep);
              if (visited.insert(ie->getSrcID()).second)
                wl.push({ie->getSrcID(), g});
            }
          }
        }
      }
    }
    return false;
  }

  // =========================================================================
                                            
  // =========================================================================
  bool computeParamMutability(const SVFFunction *func, NodeID paramID,
                              const std::set<NodeID> &valueSet,
                              bool isStructPtr) {
                                                     
    if (pointeeMutated(valueSet))
      return true;
                                                    
                                                   
                                       
                                          
                                                       
                                               
                                             
    if (isStructPtr) {
      auto sit = scopeWrittenObjs.find(func);
      if (sit != scopeWrittenObjs.end() && !sit->second.empty())
        for (NodeID o : ander->getPts(paramID))
          if (sit->second.count(o))
            return true;
    }
                                                     
    for (const CallICFGNode *cs : callsitesByFunc[func]) {
      const SVFFunction *callee = SVFUtil::getCallee(cs->getCallSite());
      if (!callee || callee->isDeclaration())
        continue;
      auto cit = funcParams.find(callee);
      if (cit == funcParams.end())
        continue;
      for (const SVFStmt *stmt : pag->getSVFStmtList(cs)) {
        const CallPE *callPE = SVFUtil::dyn_cast<CallPE>(stmt);
        if (!callPE || !valueSet.count(callPE->getRHSVarID()))
          continue;
        auto pit = cit->second.find(callPE->getLHSVarID());
        if (pit != cit->second.end() && pit->second.mutability == "Mutable")
          return true;
      }
    }
    return false;
  }

  // =========================================================================
                                          
  // =========================================================================
  void analyzeStructMemberUsage(const SVFFunction *func,
                                const std::set<NodeID> &valueSet,
                                ParamInfo &info) {
    if (info.structTypeName.empty())
      return;            

                             
    for (NodeID v : valueSet) {
      const PAGNode *n = pag->getGNode(v);
      if (!n)
        continue;
      for (const PAGEdge *e : n->getOutEdges()) {
        const GepStmt *gep = SVFUtil::dyn_cast<GepStmt>(e);
        if (!gep)
          continue;
        const AccessPath &ap = gep->getAccessPath();
        if (ap.isConstantOffset())
          info.structFields.insert(ap.getConstantStructFldIdx());
        else
          info.structVariantIndex = true;
      }
    }

                                                  
    for (const CallICFGNode *cs : callsitesByFunc[func]) {
      const SVFFunction *callee = SVFUtil::getCallee(cs->getCallSite());
      if (!callee || callee->isDeclaration())
        continue;
      auto cit = funcParams.find(callee);
      if (cit == funcParams.end())
        continue;
      for (const SVFStmt *stmt : pag->getSVFStmtList(cs)) {
        const CallPE *callPE = SVFUtil::dyn_cast<CallPE>(stmt);
        if (!callPE || !valueSet.count(callPE->getRHSVarID()))
          continue;
        auto pit = cit->second.find(callPE->getLHSVarID());
        if (pit == cit->second.end())
          continue;
        if (pit->second.structTypeName != info.structTypeName)
          continue;
        info.structFields.insert(pit->second.structFields.begin(),
                                 pit->second.structFields.end());
        info.structVariantIndex |= pit->second.structVariantIndex;
      }
    }
  }

                                                          
  std::string structMemberUsageStr(const ParamInfo &info) {
    if (info.structTypeName.empty())
      return "";
    if (info.structFields.empty() && !info.structVariantIndex)
      return "No struct members accessed";
    std::string s = info.structTypeName + ": {";
    bool first = true;
    for (int f : info.structFields) {
      if (!first)
        s += ", ";
      s += "field_" + std::to_string(f);
      first = false;
    }
    if (info.structVariantIndex) {
      if (!first)
        s += ", ";
      s += "variant_index";
    }
    return s + "}";
  }

  // =========================================================================
                                                             
  // =========================================================================
                                                             
                                           
                                                       
                                                      
                                                      
                                                             
                                                
             
  void collectCallsites(const SVFFunction *func,
                        std::map<NodeID, ParamInfo> &params) {
    PTACallGraphNode *cgn = callgraph->getCallGraphNode(func);
    if (!cgn)
      return;
                                          
    std::map<NodeID, unsigned> paramIdx;
    for (unsigned i = 0; i < func->arg_size(); ++i) {
      const SVFArgument *a = func->getArg(i);
      if (a->getType()->isPointerTy())
        paramIdx[pag->getValueNode(a)] = i;
    }
    for (PTACallGraphEdge *edge : cgn->getInEdges()) {
      for (const CallICFGNode *cs : edge->getDirectCalls()) {
        const SVFFunction *caller = cs->getCaller();
        if (!caller || caller->isDeclaration())
          continue;
        const SVFInstruction *callInst = cs->getCallSite();

                                                        
                                                               
                                                          
                                                          
                                              
        std::vector<std::pair<NodeID, NodeID>> ptrArgsAtCS;
        for (const SVFStmt *stmt : pag->getSVFStmtList(cs)) {
          const CallPE *callPE = SVFUtil::dyn_cast<CallPE>(stmt);
          if (!callPE)
            continue;
          if (params.find(callPE->getLHSVarID()) == params.end())
            continue;
          ptrArgsAtCS.emplace_back(callPE->getLHSVarID(),
                                    callPE->getRHSVarID());
        }

        for (const SVFStmt *stmt : pag->getSVFStmtList(cs)) {
          const CallPE *callPE = SVFUtil::dyn_cast<CallPE>(stmt);
          if (!callPE)
            continue;
          auto pit = params.find(callPE->getLHSVarID());          
          if (pit == params.end())
            continue;
          NodeID actualI = callPE->getRHSVarID();
          NodeID selfLHS = callPE->getLHSVarID();

                                                          
                                                             
                             
          const PointsTo &ptsI = ander->getPts(actualI);
          bool sameObj = false;
          for (auto &lr : ptrArgsAtCS) {
            if (lr.first == selfLHS)
              continue;                    
            const PointsTo &ptsJ = ander->getPts(lr.second);
            if (ptsI.intersects(ptsJ)) {
              sameObj = true;
              break;
            }
          }

          CallsiteInfo ci;
          ci.callStmt = callInst ? callInst->toString() : "";
          ci.callerArgId = traceToCallerParam(actualI, caller);
          ci.sameObject = sameObj;
          auto [pt, pto] = pointeeStrings(func, paramIdx[pit->first]);
          ci.pointsToType = pt;
          ci.pointsTo = pto;
          pit->second.callsites.push_back(ci);
        }
      }
    }
  }

                                     
                                                          
                                                          
                      
  NodeID traceToCallerParam(NodeID actualArg, const SVFFunction *caller) {
    std::set<NodeID> callerParams;
    for (unsigned i = 0; i < caller->arg_size(); ++i) {
      const SVFArgument *a = caller->getArg(i);
      if (a->getType()->isPointerTy())
        callerParams.insert(pag->getValueNode(a));
    }
    if (callerParams.empty())
      return 0;
    std::set<NodeID> visited{actualArg};
    std::queue<NodeID> wl;
    wl.push(actualArg);
    int budget = 8000;
    while (!wl.empty() && budget-- > 0) {
      NodeID cur = wl.front();
      wl.pop();
      if (callerParams.count(cur))
        return cur;
      const PAGNode *n = pag->getGNode(cur);
      if (!n)
        continue;
      for (const PAGEdge *e : n->getInEdges()) {
        auto k = e->getEdgeKind();
        if (k == PAGEdge::Copy || k == PAGEdge::Gep) {
          if (visited.insert(e->getSrcID()).second)
            wl.push(e->getSrcID());
        } else if (k == PAGEdge::Load) {
          NodeID q = e->getSrcID();
          const PAGNode *qn = pag->getGNode(q);
          if (qn && isAllocaNode(qn))
            for (const PAGEdge *se : qn->getInEdges())
              if (se->getEdgeKind() == PAGEdge::Store)
                if (visited.insert(se->getSrcID()).second)
                  wl.push(se->getSrcID());
        }
      }
    }
    return 0;
  }

  // =========================================================================
          
  // =========================================================================
  void analyzeReturn(const SVFFunction *func) {
    if (!func->getReturnType()->isPointerTy())
      return;
    if (!pag->funHasRet(func))
      return;
    const SVFVar *retVar = pag->getFunRet(func);
    if (!retVar)
      return;
    NodeID retID = retVar->getId();

    ReturnInfo ri;
    ri.returnID = retID;
    ri.funcName = func->getName();

                                                 
                                                    
                                          
    // → Borrowed。
    ri.ownership = flowsFromAllocation(retID) ? "Owning" : "Borrowed";

                                            
                    
    ri.lifeResult = computeReturnLife(func, retID);

                                      
    ri.mutability = computeReturnMutability(func) ? "Mutable" : "Immutable";

                           
    ri.nullability = "Nullable";

    funcReturns[func] = ri;
  }

                                                  
  std::string computeReturnLife(const SVFFunction *func, NodeID retID) {
    std::map<NodeID, std::string> paramNames;
    for (unsigned i = 0; i < func->arg_size(); ++i) {
      const SVFArgument *a = func->getArg(i);
      if (a->getType()->isPointerTy())
        paramNames[pag->getValueNode(a)] = a->getName();
    }
    if (paramNames.empty())
      return "No_Depends";

    std::vector<std::string> deps;
    std::set<NodeID> visited{retID};
    std::queue<NodeID> wl;
    wl.push(retID);
    int budget = 20000;
    while (!wl.empty() && budget-- > 0) {
      NodeID cur = wl.front();
      wl.pop();
      auto pn = paramNames.find(cur);
      if (pn != paramNames.end() &&
          std::find(deps.begin(), deps.end(), pn->second) == deps.end())
        deps.push_back(pn->second);
      const PAGNode *n = pag->getGNode(cur);
      if (!n)
        continue;
      for (const PAGEdge *e : n->getInEdges()) {
        auto k = e->getEdgeKind();
        if (k == PAGEdge::Copy || k == PAGEdge::Ret || k == PAGEdge::Phi ||
            k == PAGEdge::Select || k == PAGEdge::Gep) {
          if (visited.insert(e->getSrcID()).second)
            wl.push(e->getSrcID());
        } else if (k == PAGEdge::Load) {
          NodeID q = e->getSrcID();
          const PAGNode *qn = pag->getGNode(q);
          if (qn && isAllocaNode(qn))
            for (const PAGEdge *se : qn->getInEdges())
              if (se->getEdgeKind() == PAGEdge::Store)
                if (visited.insert(se->getSrcID()).second)
                  wl.push(se->getSrcID());
        }
      }
    }
    if (deps.empty())
      return "No_Depends";
    std::string s = "Return depends on `";
    for (size_t i = 0; i < deps.size(); ++i) {
      s += deps[i];
      if (i + 1 < deps.size())
        s += ", ";
    }
    return s + "`";
  }

                                                
              
  bool computeReturnMutability(const SVFFunction *func) {
    PTACallGraphNode *cgn = callgraph->getCallGraphNode(func);
    if (!cgn)
      return false;
    for (PTACallGraphEdge *edge : cgn->getInEdges()) {
      for (const CallICFGNode *cs : edge->getDirectCalls()) {
        const SVFInstruction *callInst = cs->getCallSite();
        if (!callInst || !pag->hasValueNode(callInst))
          continue;
        std::set<NodeID> vs = collectValueSet(pag->getValueNode(callInst));
        if (pointeeMutated(vs))
          return true;
      }
    }
    return false;
  }

  // =========================================================================
                              
  // =========================================================================
  void generateReport(const std::string &outputFile) {
    nlohmann::json reportJson;
    auto now = std::chrono::system_clock::now();
    auto t = std::chrono::system_clock::to_time_t(now);
    std::string ts = std::ctime(&t);
    if (!ts.empty() && ts.back() == '\n')
      ts.pop_back();
    reportJson["timestamp"] = ts;
    reportJson["analysis_type"] = "函数参数与返回值分析";

    std::set<const SVFFunction *> allFns;
    for (const auto &kv : funcParams)
      allFns.insert(kv.first);
    for (const auto &kv : funcReturns)
      allFns.insert(kv.first);

    nlohmann::json functionsJson;
    for (const SVFFunction *func : allFns) {
      nlohmann::json funcJson;
      funcJson["function_name"] = func->getName();
      funcJson["is_declaration"] = func->isDeclaration();

      // ── parameters ──────────────────────────────────────────────
      nlohmann::json paramsJson = nlohmann::json::array();
      auto pit = funcParams.find(func);
      if (pit != funcParams.end()) {
        for (const auto &kv : pit->second) {
          const ParamInfo &info = kv.second;
          nlohmann::json pj;
          pj["param_id"] = info.paramID;
          pj["param_name"] = info.paramName;
          pj["mutability"] = info.mutability;
          pj["nullability"] = info.nullability;
          pj["ownership"] = info.ownership;
          pj["count"] = info.countResult;

          std::string smu = structMemberUsageStr(info);
          if (!smu.empty()) {
            nlohmann::json smj;
            if (smu == "No struct members accessed") {
              smj["accessed_fields"] = nlohmann::json::array();
            } else {
              size_t colon = smu.find(": {");
              std::string sname = smu.substr(0, colon);
              std::string body =
                  smu.substr(colon + 3, smu.size() - colon - 4);
              smj["struct_type"] = sname;
              nlohmann::json fields = nlohmann::json::array();
              size_t start = 0;
              while (start < body.size()) {
                size_t comma = body.find(", ", start);
                std::string tok = body.substr(
                    start, comma == std::string::npos ? std::string::npos
                                                      : comma - start);
                if (!tok.empty())
                  fields.push_back(tok);
                if (comma == std::string::npos)
                  break;
                start = comma + 2;
              }
              smj["accessed_fields"] = fields;
            }
            pj["struct_member_usage"] = smj;
          }

          nlohmann::json csJson = nlohmann::json::array();
          for (const CallsiteInfo &ci : info.callsites) {
            nlohmann::json cj;
            cj["call_stmt"] = ci.callStmt;
            cj["caller_arg_id"] = ci.callerArgId;
            cj["same_object"] = ci.sameObject;
            cj["points_to_type"] = ci.pointsToType;
            cj["points_to"] = ci.pointsTo;
            csJson.push_back(cj);
          }
          pj["callsites"] = csJson;
          paramsJson.push_back(pj);
        }
      }
      funcJson["parameters"] = paramsJson;

      // ── returns ─────────────────────────────────────────────────
      nlohmann::json returnsJson = nlohmann::json::array();
      auto rit = funcReturns.find(func);
      if (rit != funcReturns.end()) {
        const ReturnInfo &ri = rit->second;
        nlohmann::json rj;
        rj["return_id"] = ri.returnID;
        rj["return_name"] = ri.returnName;
        rj["mutability"] = ri.mutability;
        rj["nullability"] = ri.nullability;
        rj["ownership"] = ri.ownership;
        rj["life_result"] = ri.lifeResult;
        returnsJson.push_back(rj);
      }
      funcJson["returns"] = returnsJson;

      functionsJson[func->getName()] = funcJson;
    }
    reportJson["function_analysis"] = functionsJson;

    std::ofstream outFile(outputFile);
    if (!outFile.is_open()) {
      std::cerr << "无法打开输出文件: " << outputFile << std::endl;
      return;
    }
    outFile << reportJson.dump(4) << std::endl;
    outFile.close();
    std::cout << "分析报告已保存到: " << outputFile << std::endl;
  }
};

int main(int argc, char **argv) {
  std::cout << "SVF 函数参数/返回值指针分析开始" << std::endl;

  std::vector<std::string> irFiles;
  for (int i = 1; i < argc; ++i)
    irFiles.emplace_back(argv[i]);

  std::cout << "初始化SVF..." << std::endl;
  SVFModule *svfModule =
      LLVMModuleSet::getLLVMModuleSet()->buildSVFModule(irFiles);

  std::cout << "构建SVFIR..." << std::endl;
  SVFIRBuilder builder(svfModule);
  SVFIR *pag = builder.build();

                                               
                                                     
                                              
                 
  std::cout << "执行 Andersen 指针分析..." << std::endl;
  AndersenWaveDiff *ander = AndersenWaveDiff::createAndersenWaveDiff(pag);

  FunctionParamAnalyzer analyzer(pag, ander);
  analyzer.run();
  analyzer.generateReport("func_analysis_report.json");

  SVFIR::releaseSVFIR();
  LLVMModuleSet::releaseLLVMModuleSet();

  std::cout << "函数参数/返回值分析完成." << std::endl;
  return 0;
}
