# 密码学 Cryptography

## 密码学基础 Foundations of Cryptography



### 1.对称加密与哈希函数 Symmetric encryption and hash functions

对称加密与哈希函数的核心原理：为实现随机性 

The core principle of symmetric encryption and hash functions: to achieve randomness

- 线性变换：每 **bit** **的输入发生变换，影响** 50%输出，密文随机性高。 Linear transformation: Every **bit** of input is transformed, affecting 50% of the output, and the ciphertext is highly random.

- 非线性变换：S盒子，抵抗解方程组攻击、延展攻击。Nonlinear transformation: S-box, resists attacks from solving systems of equations and extension attacks.

​		**延展性：**基于已有的密文，计算新密文，且新密文能正确解密。Extensibility: Based on the existing ciphertext, a new ciphertext is calculated, and the new ciphertext can be decrypted correctly.

举例：同态加密具有延展性：

Example: Homomorphic encryption is scalable:

<img src="images/image-20240716224244840.png" alt="image-20240716224244840" style="zoom:50%;" />

- 轮密钥加轮常量加：添加一些随机密钥或常量，提高信息商。Round key plus round constant plus: Add some random keys or constants to improve the information quotient.

#### 1 对称加密 Symmetric encryption

##### 1.1 基本概念 Basic concepts

<img src="images/image-20240716224706741.png" alt="image-20240716224706741"  />

**初始化：**双方共享一个保密随机数$K$ ，或使用相同的设备生成一个相同的随机数 $K$。**Initialization:** Both parties share a secret random number $K$, or use the same device to generate the same random number $K$.

**加密：**发送方使用设备生成随机数$K$ ，对消息 $X$，如下计算：**Encryption:** The sender uses the device to generate a random number $K$ and calculates the message $X$ as follows:
$$
Y:=K\bigoplus X
$$
发送密文 $Y$。Send ciphertext $Y$.

**解密：**接收方接收密文 $Y$，使用设备生成随机数 $K$，如下计算 **Decryption:** The receiver receives the ciphertext $Y$ and uses the device to generate a random number $K$, which is calculated as follows
$$
X:=Y\bigoplus K
$$
获得消息$X$。Get message $X$.

**双方如何容易地**共享保密随机数 *K* ？信道是否一定要安全，不安全可以吗？

How can both parties easily share a secret random number *K*? Does the channel have to be secure? Can it be insecure?

**答：可以的，这是**公钥密码学的任务。

**Answer: Yes, this is the task of **public key cryptography.

##### 1.2 高级加密标准AES  Advanced Encryption Standard AES



![image-20240716232313143](images/image-20240716232313143.png)



### 2.公钥加密与数字签名







### 3.密码协议：承诺、零知识证明、秘钥协商



### 4.同态加密



## ECDSA多签系列

### 1.Li17两方签名



 ### 2.GG18多方签名



### 3.GG20多方签名



### 4.CMP20多方签名



### 5.DKLs18两方/20多方签名



### 6.Schnorr/EdDSA多方签名



## zk系列

### 1.Groth16证明系统



### 2.Plonk证明系统



### 3.UltraPlonk证明系统



### 4.SHA256查找方法



### 5.Halo2证明系统



### 6.zkSTARK证明系统



