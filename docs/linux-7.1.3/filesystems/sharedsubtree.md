
## 鍏变韩瀛愭爲锛圫hared Subtrees锛?


 1) 姒傝堪锛圤verview锛?
 2) 鐗规€э紙Features锛?
 3) 璁剧疆鎸傝浇鐘舵€侊紙Setting mount states锛?
 4) 浣跨敤鍦烘櫙锛圲se-case锛?
 5) 璇︾粏璇箟锛圖etailed semantics锛?
 6) 娴嬮獙锛圦uiz锛?
 7) 甯歌闂锛團AQ锛?
 8) 瀹炵幇锛圛mplementation锛?


### 1) 姒傝堪锛圤verview锛?


鑰冭檻浠ヤ笅鍦烘櫙锛?

鏌愪釜杩涚▼鎯宠鍏嬮殕锛坈lone锛夊畠鑷繁鐨勫懡鍚嶇┖闂达紙namespace锛夛紝浣嗕粛甯屾湜璁块棶鏈€杩戞寕杞界殑 CD銆傚叡浜瓙鏍戣涔夛紙shared subtree semantics锛夋彁渚涗簡瀹炵幇涓婅堪鐩爣鎵€闇€鐨勬満鍒躲€?

瀹冧负璇稿姣忕敤鎴峰懡鍚嶇┖闂达紙per-user-namespace锛夊拰鐗堟湰鍖栨枃浠剁郴缁燂紙versioned filesystem锛夌瓑鐗规€ф彁渚涗簡蹇呰鐨勬瀯寤烘ā鍧椼€?

### 2) 鐗规€э紙Features锛?


鍏变韩瀛愭爲鎻愪緵浜嗗洓绉嶄笉鍚岀被鍨嬬殑鎸傝浇锛坢ount锛夛紱鍑嗙‘鍦拌锛屾槸 struct vfsmount 鐨勫洓绉嶄笉鍚岀姸鎬侊細


a) **鍏变韩鎸傝浇锛坰hared mount锛?* 鍙互琚鍒跺埌浠绘剰澶氫釜鎸傝浇鐐癸紝骞朵笖鎵€鏈夊壇鏈缁堜繚鎸佸畬鍏ㄤ竴鑷淬€?

   绀轰緥濡備笅锛?


```

     # mount --make-shared /mnt

   .. note::
      mount(8) command now supports the --make-shared flag,
      so the sample 'smount' program is no longer needed and has been
      removed.

   ::

     # mount --bind /mnt /tmp

   The above command replicates the mount at /mnt to the mountpoint /tmp
   and the contents of both the mounts remain identical.

   ::

     #ls /mnt
     a b c

     #ls /tmp
     a b c

   Now let's say we mount a device at /tmp/a::

     # mount /dev/sd0  /tmp/a

     # ls /tmp/a
     t1 t2 t3

     # ls /mnt/a
     t1 t2 t3

   Note that the mount has propagated to the mount at /mnt as well.

   And the same is true even when /dev/sd0 is mounted on /mnt/a. The
   contents will be visible under /tmp/a too.


```
b) **浠庡睘鎸傝浇锛坰lave mount锛?* 绫讳技浜庡叡浜寕杞斤紝浣嗘寕杞斤紙mount锛夊拰鍗歌浇锛坲mount锛変簨浠跺彧鍚戝畠鍗曞悜浼犳挱銆?

   鎵€鏈変粠灞炴寕杞介兘鏈変竴涓富鎸傝浇锛坢aster mount锛夛紝鑰屼富鎸傝浇鏈韩鏄竴涓叡浜寕杞姐€?

   绀轰緥濡備笅锛?


```

     # mount --make-shared /mnt

   Let's bind mount /mnt to /tmp::

     # mount --bind /mnt /tmp

   the new mount at /tmp becomes a shared mount and it is a replica of
   the mount at /mnt.

   Now let's make the mount at /tmp; a slave of /mnt::

     # mount --make-slave /tmp

   let's mount /dev/sd0 on /mnt/a::

     # mount /dev/sd0 /mnt/a

     # ls /mnt/a
     t1 t2 t3

     # ls /tmp/a
     t1 t2 t3

   Note the mount event has propagated to the mount at /tmp

   However let's see what happens if we mount something on the mount at
   /tmp::

     # mount /dev/sd1 /tmp/b

     # ls /tmp/b
     s1 s2 s3

     # ls /mnt/b

   Note how the mount event has not propagated to the mount at
   /mnt


```
c) **绉佹湁鎸傝浇锛坧rivate mount锛?* 鏃笉杞彂涔熶笉鎺ユ敹浼犳挱銆?

   杩欐槸鎴戜滑鎵€鐔熸倝鐨勬寕杞界被鍨嬶紝涔熸槸榛樿绫诲瀷銆?


d) **涓嶅彲缁戝畾鎸傝浇锛坲nbindable mount锛?* 椤惧悕鎬濅箟锛屾槸涓€绉嶆棤娉曡缁戝畾鎸傝浇鐨勭鏈夋寕杞姐€?


```

     # mount --make-unbindable /mnt

   Let's try to bind mount this mount somewhere else::

     # mount --bind /mnt /tmp mount: wrong fs type, bad option, bad
     superblock on /mnt, or too many mounted file systems

   Binding a unbindable mount is a invalid operation.


```
### 3) 璁剧疆鎸傝浇鐘舵€侊紙Setting mount states锛?


鍙互浣跨敤 mount 鍛戒护锛坲til-linux 杞欢鍖咃級鏉ヨ缃寕杞界姸鎬侊細


```

    mount --make-shared mountpoint
    mount --make-slave mountpoint
    mount --make-private mountpoint
    mount --make-unbindable mountpoint


```
### 4) 浣跨敤鍦烘櫙锛圲se cases锛?


A) 鏌愪釜杩涚▼鎯宠鍏嬮殕鑷繁鐨勫懡鍚嶇┖闂达紝浣嗕粛甯屾湜璁块棶鏈€杩戞寕杞界殑 CD銆?

   瑙ｅ喅鏂规锛?


```

     mount --bind /cdrom /cdrom
     mount --make-shared /cdrom

   Now any process that clones off a new namespace will have a
   mount at /cdrom which is a replica of the same mount in the
   parent namespace.

   So when a CD is inserted and mounted at /cdrom that mount gets
   propagated to the other mount at /cdrom in all the other clone
   namespaces.

```
B) 鏌愪釜杩涚▼甯屾湜鑷繁鐨勬寕杞藉鍏朵粬浠讳綍杩涚▼涓嶅彲瑙侊紝浣嗕粛鑳界湅鍒扮郴缁熺殑鍏朵粬鎸傝浇銆?

   瑙ｅ喅鏂规锛?

   棣栧厛锛岀鐞嗗憳鍙互灏嗘暣涓寕杞芥爲鏍囪涓?


```

     mount --make-rshared /

   A new process can clone off a new namespace. And mark some part
   of its namespace as slave::

     mount --make-rslave /myprivatetree

   Hence forth any mounts within the /myprivatetree done by the
   process will not show up in any other namespace. However mounts
   done in the parent namespace under /myprivatetree still shows
   up in the process's namespace.


```
闄や簡涓婅堪璇箟涔嬪锛岃鐗规€ц繕涓鸿В鍐充互涓嬮棶棰樻彁渚涗簡鏋勫缓妯″潡锛?

C) 姣忕敤鎴峰懡鍚嶇┖闂达紙Per-user namespace锛?

   涓婅堪璇箟鎻愪緵浜嗕竴绉嶈法鍛藉悕绌洪棿鍏变韩鎸傝浇鐨勬柟寮忋€備絾鍛藉悕绌洪棿鏄笌杩涚▼鐩稿叧鑱旂殑銆傚鏋滃懡鍚嶇┖闂磋瀹炵幇涓轰竴娴佸璞★紝骞舵彁渚涚敤鎴?API 鏉ュ皢鍛藉悕绌洪棿涓庣敤鎴?ID 鍏宠仈/瑙ｉ櫎鍏宠仈锛岄偅涔堟瘡涓敤鎴烽兘鍙互鎷ユ湁浠?濂硅嚜宸辩殑鍛藉悕绌洪棿锛屽苟鏍规嵁鍏堕渶姹傝繘琛屽畾鍒躲€傝繖闇€瑕佸湪 PAM 涓彁渚涙敮鎸併€?

D) 鐗堟湰鍖栨枃浠讹紙Versioned files锛?

   濡傛灉鏁翠釜鎸傝浇鏍戝湪澶氫釜浣嶇疆鍙锛岄偅涔堝簳灞傜殑鐗堟湰鍖栨枃浠剁郴缁熷彲浠ユ牴鎹敤浜庤闂鏂囦欢鐨勮矾寰勶紝杩斿洖璇ユ枃浠剁殑涓嶅悓鐗堟湰銆?


```

       mount --make-shared /
       mount --rbind / /view/v1
       mount --rbind / /view/v2
       mount --rbind / /view/v3
       mount --rbind / /view/v4

    and if /usr has a versioning filesystem mounted, then that
    mount appears at /view/v1/usr, /view/v2/usr, /view/v3/usr and
    /view/v4/usr too

    A user can request v3 version of the file /usr/fs/namespace.c
    by accessing /view/v3/usr/fs/namespace.c . The underlying
    versioning filesystem can then decipher that v3 version of the
    filesystem is being requested and return the corresponding
    inode.

```
### 5) 璇︾粏璇箟锛圖etailed semantics锛?

鏈妭瑙ｉ噴浜?bind銆乺bind銆乵ove銆乵ount銆乽mount 浠ュ強鍏嬮殕鍛藉悕绌洪棿锛坈lone-namespace锛夋搷浣滅殑璇︾粏璇箟銆?

   鏈枃妗ｄ腑锛屽崟璇?'vfsmount' 鍜屽悕璇?'mount' 琚敤鏉ユ寚浠ｅ悓涓€涓蹇点€?

a) 鎸傝浇鐘舵€侊紙Mount states锛?

   **浼犳挱浜嬩欢锛坧ropagation event锛?* 瀹氫箟涓哄湪鏌愪釜 vfsmount 涓婁骇鐢熴€佸苟瀵艰嚧鍏朵粬 vfsmount 涓婂彂鐢熸寕杞芥垨鍗歌浇鍔ㄤ綔鐨勪簨浠躲€?

   **瀵圭瓑缁勶紙peer group锛?* 瀹氫箟涓轰竴缁勭浉浜掍紶鎾簨浠剁殑 vfsmount銆?

   涓€涓粰瀹氱殑鎸傝浇鍙互澶勪簬浠ヤ笅鐘舵€佷箣涓€锛?

   (1) 鍏变韩鎸傝浇锛圫hared mounts锛?

       **鍏变韩鎸傝浇锛坰hared mount锛?* 瀹氫箟涓哄睘浜庢煇涓绛夌粍鐨?vfsmount銆?


```

         mount --make-shared /mnt
         mount --bind /mnt /tmp

       The mount at /mnt and that at /tmp are both shared and belong
       to the same peer group. Anything mounted or unmounted under
       /mnt or /tmp reflect in all the other mounts of its peer
       group.


   (2) Slave mounts

       A **slave mount** is defined as a vfsmount that receives
       propagation events and does not forward propagation events.

       A slave mount as the name implies has a master mount from which
       mount/unmount events are received. Events do not propagate from
       the slave mount to the master.  Only a shared mount can be made
       a slave by executing the following command::

         mount --make-slave mount

       A shared mount that is made as a slave is no more shared unless
       modified to become shared.

   (3) Shared and Slave

       A vfsmount can be both **shared** as well as **slave**.  This state
       indicates that the mount is a slave of some vfsmount, and
       has its own peer group too.  This vfsmount receives propagation
       events from its master vfsmount, and also forwards propagation
       events to its 'peer group' and to its slave vfsmounts.

       Strictly speaking, the vfsmount is shared having its own
       peer group, and this peer-group is a slave of some other
       peer group.

       Only a slave vfsmount can be made as 'shared and slave' by
       either executing the following command::

         mount --make-shared mount

       or by moving the slave vfsmount under a shared vfsmount.

   (4) Private mount

       A **private mount** is defined as vfsmount that does not
       receive or forward any propagation events.

   (5) Unbindable mount

       A **unbindable mount** is defined as vfsmount that does not
       receive or forward any propagation events and cannot
       be bind mounted.


       State diagram:

       The state diagram below explains the state transition of a mount,
       in response to various commands::

            -----------------------------------------------------------------------
            |             |make-shared |  make-slave  | make-private |make-unbindab|
            --------------|------------|--------------|--------------|-------------|
            |shared       |shared      |*slave/private|   private    | unbindable  |
            |             |            |              |              |             |
            |-------------|------------|--------------|--------------|-------------|
            |slave        |shared      | **slave      |    private   | unbindable  |
            |             |and slave   |              |              |             |
            |-------------|------------|--------------|--------------|-------------|
            |shared       |shared      | slave        |    private   | unbindable  |
            |and slave    |and slave   |              |              |             |
            |-------------|------------|--------------|--------------|-------------|
            |private      |shared      |  **private   |    private   | unbindable  |
            |-------------|------------|--------------|--------------|-------------|
            |unbindable   |shared      |**unbindable  |    private   | unbindable  |
            ------------------------------------------------------------------------

            * if the shared mount is the only mount in its peer group, making it
            slave, makes it private automatically. Note that there is no master to
            which it can be slaved to.

            ** slaving a non-shared mount has no effect on the mount.

       Apart from the commands listed below, the 'move' operation also changes
       the state of a mount depending on type of the destination mount. Its
       explained in section 5d.

```
b) 缁戝畾璇箟锛圔ind semantics锛?


```

     mount --bind A/a  B/b

   where 'A' is the source mount, 'a' is the dentry in the mount 'A', 'B'
   is the destination mount and 'b' is the dentry in the destination mount.

   The outcome depends on the type of mount of 'A' and 'B'. The table
   below contains quick reference::

            --------------------------------------------------------------------------
            |         BIND MOUNT OPERATION                                           |
            |************************************************************************|
            |source(A)->| shared      |       private  |       slave    | unbindable |
            | dest(B)  |              |                |                |            |
            |   |      |              |                |                |            |
            |   v      |              |                |                |            |
            |************************************************************************|
            |  shared  | shared       |     shared     | shared & slave |  invalid   |
            |          |              |                |                |            |
            |non-shared| shared       |      private   |      slave     |  invalid   |
            **************************************************************************

   Details:

   1. 'A' is a shared mount and 'B' is a shared mount. A new mount 'C'
      which is clone of 'A', is created. Its root dentry is 'a' . 'C' is
      mounted on mount 'B' at dentry 'b'. Also new mount 'C1', 'C2', 'C3' ...
      are created and mounted at the dentry 'b' on all mounts where 'B'
      propagates to. A new propagation tree containing 'C1',..,'Cn' is
      created. This propagation tree is identical to the propagation tree of
      'B'.  And finally the peer-group of 'C' is merged with the peer group
      of 'A'.

   2. 'A' is a private mount and 'B' is a shared mount. A new mount 'C'
      which is clone of 'A', is created. Its root dentry is 'a'. 'C' is
      mounted on mount 'B' at dentry 'b'. Also new mount 'C1', 'C2', 'C3' ...
      are created and mounted at the dentry 'b' on all mounts where 'B'
      propagates to. A new propagation tree is set containing all new mounts
      'C', 'C1', .., 'Cn' with exactly the same configuration as the
      propagation tree for 'B'.

   3. 'A' is a slave mount of mount 'Z' and 'B' is a shared mount. A new
      mount 'C' which is clone of 'A', is created. Its root dentry is 'a' .
      'C' is mounted on mount 'B' at dentry 'b'. Also new mounts 'C1', 'C2',
      'C3' ... are created and mounted at the dentry 'b' on all mounts where
      'B' propagates to. A new propagation tree containing the new mounts
      'C','C1',..  'Cn' is created. This propagation tree is identical to the
      propagation tree for 'B'. And finally the mount 'C' and its peer group
      is made the slave of mount 'Z'.  In other words, mount 'C' is in the
      state 'slave and shared'.

   4. 'A' is a unbindable mount and 'B' is a shared mount. This is a
      invalid operation.

   5. 'A' is a private mount and 'B' is a non-shared(private or slave or
      unbindable) mount. A new mount 'C' which is clone of 'A', is created.
      Its root dentry is 'a'. 'C' is mounted on mount 'B' at dentry 'b'.

   6. 'A' is a shared mount and 'B' is a non-shared mount. A new mount 'C'
      which is a clone of 'A' is created. Its root dentry is 'a'. 'C' is
      mounted on mount 'B' at dentry 'b'.  'C' is made a member of the
      peer-group of 'A'.

   7. 'A' is a slave mount of mount 'Z' and 'B' is a non-shared mount. A
      new mount 'C' which is a clone of 'A' is created. Its root dentry is
      'a'.  'C' is mounted on mount 'B' at dentry 'b'. Also 'C' is set as a
      slave mount of 'Z'. In other words 'A' and 'C' are both slave mounts of
      'Z'.  All mount/unmount events on 'Z' propagates to 'A' and 'C'. But
      mount/unmount on 'A' do not propagate anywhere else. Similarly
      mount/unmount on 'C' do not propagate anywhere else.

   8. 'A' is a unbindable mount and 'B' is a non-shared mount. This is a
      invalid operation. A unbindable mount cannot be bind mounted.

```
c) 閫掑綊缁戝畾璇箟锛圧bind semantics锛?

   rbind 涓?bind 鐩稿悓銆俠ind 澶嶅埗鎸囧畾鐨勬寕杞姐€俽bind 澶嶅埗灞炰簬鎸囧畾鎸傝浇鐨勬爲涓殑鎵€鏈夋寕杞姐€俽bind 鎸傝浇灏辨槸瀵规爲涓墍鏈夋寕杞藉簲鐢ㄧ殑 bind 鎸傝浇銆?

   濡傛灉琚?rbind 鐨勬簮鏍戜腑鍖呭惈涓€浜涗笉鍙粦瀹氭寕杞斤紝閭ｄ箞杩欎簺涓嶅彲缁戝畾鎸傝浇涔嬩笅鐨勫瓙鏍戜細鍦ㄦ柊浣嶇疆琚壀闄ゃ€?

   渚嬪锛?


```

                A
              /   \
              B   C
             / \ / \
             D E F G

   Let's say all the mount except the mount C in the tree are
   of a type other than unbindable.

   If this tree is rbound to say Z

   We will have the following tree at the new location::

                Z
                |
                A'
               /
              B'                Note how the tree under C is pruned
             / \                in the new location.
            D' E'



```
d) 绉诲姩璇箟锛圡ove semantics锛?


```

     mount --move A  B/b

   where 'A' is the source mount, 'B' is the destination mount and 'b' is
   the dentry in the destination mount.

   The outcome depends on the type of the mount of 'A' and 'B'. The table
   below is a quick reference::

            ---------------------------------------------------------------------------
            |                   MOVE MOUNT OPERATION                                 |
            |**************************************************************************
            | source(A)->| shared      |       private  |       slave    | unbindable |
            | dest(B)  |               |                |                |            |
            |   |      |               |                |                |            |
            |   v      |               |                |                |            |
            |**************************************************************************
            |  shared  | shared        |     shared     |shared and slave|  invalid   |
            |          |               |                |                |            |
            |non-shared| shared        |      private   |    slave       | unbindable |
            ***************************************************************************

   .. Note:: moving a mount residing under a shared mount is invalid.

   Details follow:

   1. 'A' is a shared mount and 'B' is a shared mount.  The mount 'A' is
      mounted on mount 'B' at dentry 'b'.  Also new mounts 'A1', 'A2'...'An'
      are created and mounted at dentry 'b' on all mounts that receive
      propagation from mount 'B'. A new propagation tree is created in the
      exact same configuration as that of 'B'. This new propagation tree
      contains all the new mounts 'A1', 'A2'...  'An'.  And this new
      propagation tree is appended to the already existing propagation tree
      of 'A'.

   2. 'A' is a private mount and 'B' is a shared mount. The mount 'A' is
      mounted on mount 'B' at dentry 'b'. Also new mount 'A1', 'A2'... 'An'
      are created and mounted at dentry 'b' on all mounts that receive
      propagation from mount 'B'. The mount 'A' becomes a shared mount and a
      propagation tree is created which is identical to that of
      'B'. This new propagation tree contains all the new mounts 'A1',
      'A2'...  'An'.

   3. 'A' is a slave mount of mount 'Z' and 'B' is a shared mount.  The
      mount 'A' is mounted on mount 'B' at dentry 'b'.  Also new mounts 'A1',
      'A2'... 'An' are created and mounted at dentry 'b' on all mounts that
      receive propagation from mount 'B'. A new propagation tree is created
      in the exact same configuration as that of 'B'. This new propagation
      tree contains all the new mounts 'A1', 'A2'...  'An'.  And this new
      propagation tree is appended to the already existing propagation tree of
      'A'.  Mount 'A' continues to be the slave mount of 'Z' but it also
      becomes 'shared'.

   4. 'A' is a unbindable mount and 'B' is a shared mount. The operation
      is invalid. Because mounting anything on the shared mount 'B' can
      create new mounts that get mounted on the mounts that receive
      propagation from 'B'.  And since the mount 'A' is unbindable, cloning
      it to mount at other mountpoints is not possible.

   5. 'A' is a private mount and 'B' is a non-shared(private or slave or
      unbindable) mount. The mount 'A' is mounted on mount 'B' at dentry 'b'.

   6. 'A' is a shared mount and 'B' is a non-shared mount.  The mount 'A'
      is mounted on mount 'B' at dentry 'b'.  Mount 'A' continues to be a
      shared mount.

   7. 'A' is a slave mount of mount 'Z' and 'B' is a non-shared mount.
      The mount 'A' is mounted on mount 'B' at dentry 'b'.  Mount 'A'
      continues to be a slave mount of mount 'Z'.

   8. 'A' is a unbindable mount and 'B' is a non-shared mount. The mount
      'A' is mounted on mount 'B' at dentry 'b'. Mount 'A' continues to be a
      unbindable mount.

```
e) 鎸傝浇璇箟锛圡ount semantics锛?


```

     mount device  B/b

   'B' is the destination mount and 'b' is the dentry in the destination
   mount.

   The above operation is the same as bind operation with the exception
   that the source mount is always a private mount.


```
f) 鍗歌浇璇箟锛圲nmount semantics锛?


```

     umount A

   where 'A' is a mount mounted on mount 'B' at dentry 'b'.

   If mount 'B' is shared, then all most-recently-mounted mounts at dentry
   'b' on mounts that receive propagation from mount 'B' and does not have
   sub-mounts within them are unmounted.

   Example: Let's say 'B1', 'B2', 'B3' are shared mounts that propagate to
   each other.

   let's say 'A1', 'A2', 'A3' are first mounted at dentry 'b' on mount
   'B1', 'B2' and 'B3' respectively.

   let's say 'C1', 'C2', 'C3' are next mounted at the same dentry 'b' on
   mount 'B1', 'B2' and 'B3' respectively.

   if 'C1' is unmounted, all the mounts that are most-recently-mounted on
   'B1' and on the mounts that 'B1' propagates-to are unmounted.

   'B1' propagates to 'B2' and 'B3'. And the most recently mounted mount
   on 'B2' at dentry 'b' is 'C2', and that of mount 'B3' is 'C3'.

   So all 'C1', 'C2' and 'C3' should be unmounted.

   If any of 'C2' or 'C3' has some child mounts, then that mount is not
   unmounted, but all other mounts are unmounted. However if 'C1' is told
   to be unmounted and 'C1' has some sub-mounts, the umount operation is
   failed entirely.

```
g) 鍏嬮殕鍛藉悕绌洪棿锛圕lone Namespace锛?

   涓€涓厠闅嗗嚭鐨勫懡鍚嶇┖闂村寘鍚笌鐖跺懡鍚嶇┖闂寸浉鍚岀殑鎵€鏈夋寕杞姐€?

   鍋囪 'A' 鍜?'B' 鍒嗗埆鏄埗鍛藉悕绌洪棿鍜屽瓙鍛藉悕绌洪棿涓搴旂殑鎸傝浇銆?

   濡傛灉 'A' 鏄叡浜殑锛岄偅涔?'B' 涔熸槸鍏变韩鐨勶紝骞朵笖 'A' 鍜?'B' 鐩镐簰浼犳挱銆?

   濡傛灉 'A' 鏄寕杞?'Z' 鐨勪粠灞炴寕杞斤紝閭ｄ箞 'B' 涔熸槸 'Z' 鐨勪粠灞炴寕杞姐€?

   濡傛灉 'A' 鏄鏈夋寕杞斤紝閭ｄ箞 'B' 涔熸槸绉佹湁鎸傝浇銆?

   濡傛灉 'A' 鏄笉鍙粦瀹氭寕杞斤紝閭ｄ箞 'B' 涔熸槸涓嶅彲缁戝畾鎸傝浇銆?


### 6) 娴嬮獙锛圦uiz锛?


A. 浠ヤ笅鍛戒护搴忓垪鐨勭粨鏋滄槸浠€涔堬紵


```

       mount --bind /mnt /mnt
       mount --make-shared /mnt
       mount --bind /mnt /tmp
       mount --move /tmp /mnt/1

   what should be the contents of /mnt /mnt/1 /mnt/1/1 should be?
   Should they all be identical? or should /mnt and /mnt/1 be
   identical only?


```
B. 浠ヤ笅鍛戒护搴忓垪鐨勭粨鏋滄槸浠€涔堬紵


```

       mount --make-rshared /
       mkdir -p /v/1
       mount --rbind / /v/1

   what should be the content of /v/1/v/1 be?


```
C. 浠ヤ笅鍛戒护搴忓垪鐨勭粨鏋滄槸浠€涔堬紵


```

       mount --bind /mnt /mnt
       mount --make-shared /mnt
       mkdir -p /mnt/1/2/3 /mnt/1/test
       mount --bind /mnt/1 /tmp
       mount --make-slave /mnt
       mount --make-shared /mnt
       mount --bind /mnt/1/2 /tmp1
       mount --make-slave /mnt

   At this point we have the first mount at /tmp and
   its root dentry is 1. Let's call this mount 'A'
   And then we have a second mount at /tmp1 with root
   dentry 2. Let's call this mount 'B'
   Next we have a third mount at /mnt with root dentry
   mnt. Let's call this mount 'C'

   'B' is the slave of 'A' and 'C' is a slave of 'B'
   A -> B -> C

   at this point if we execute the following command::

     mount --bind /bin /tmp/test

   The mount is attempted on 'A'

   will the mount propagate to 'B' and 'C' ?

   what would be the contents of
   /mnt/1/test be?

```
### 7) 甯歌闂锛團AQ锛?


1. 涓轰粈涔堥渶瑕佺粦瀹氭寕杞斤紵瀹冧笌绗﹀彿閾炬帴鏈変綍涓嶅悓锛?

   绗﹀彿閾炬帴鍦ㄧ洰鏍囨寕杞借鍗歌浇鎴栫Щ鍔ㄦ椂鍙兘浼氬け鏁堛€傝€岀粦瀹氭寕杞藉嵆浣垮叾浠栨寕杞借鍗歌浇鎴栫Щ鍔ㄤ篃渚濈劧瀛樺湪銆?

2. 涓轰粈涔堜笉鑳藉彧鐢?exportfs 鏉ュ疄鐜板叡浜瓙鏍戯紵

   exportfs 鏄竴绉嶅疄鐜板叡浜瓙鏍戦儴鍒嗗姛鑳界殑楂樺紑閿€鏂瑰紡銆傛垜鏃犳硶鎯宠薄鍑轰竴绉嶈兘鐢?exportfs 瀹炵幇浠庡睘鎸傝浇璇箟鐨勬柟娉曘€?

3. 涓轰粈涔堥渶瑕佷笉鍙粦瀹氭寕杞斤紵

   鍋囪鎴戜滑鎯冲湪鍚屼竴瀛愭爲鍐呯殑澶氫釜浣嶇疆澶嶅埗鎸傝浇鏍戙€?

   濡傛灉鍦ㄥ悓涓€瀛愭爲鍐呭皢涓€妫垫爲 rbind 鎸傝浇 'n' 娆★紝鎵€鍒涘缓鐨勬寕杞芥暟閲忔槸 'n' 鐨勬寚鏁板嚱鏁般€備娇鐢ㄤ笉鍙粦瀹氭寕杞芥湁鍔╀簬鍓櫎涓嶉渶瑕佺殑缁戝畾鎸傝浇銆備笅闈㈡槸涓€涓緥瀛愩€?

   step 1:
      鍋囪鏍规爲鍙湁涓や釜鐩綍


```

                                    root
                                   /    \
                                  tmp    usr

      And we want to replicate the tree at multiple
      mountpoints under /root/tmp

   step 2:
      ::


                        mount --make-shared /root

                        mkdir -p /tmp/m1

                        mount --rbind /root /tmp/m1

      the new tree now looks like this::

                                    root
                                   /    \
                                 tmp    usr
                                /
                               m1
                              /  \
                             tmp  usr
                             /
                            m1

      it has two vfsmounts

   step 3:
      ::

                            mkdir -p /tmp/m2
                            mount --rbind /root /tmp/m2

      the new tree now looks like this::

                                      root
                                     /    \
                                   tmp     usr
                                  /    \
                                m1       m2
                               / \       /  \
                             tmp  usr   tmp  usr
                             / \          /
                            m1  m2      m1
                                / \     /  \
                              tmp usr  tmp   usr
                              /        / \
                             m1       m1  m2
                            /  \
                          tmp   usr
                          /  \
                         m1   m2

                    it has 6 vfsmounts

   step 4:
      ::

                          mkdir -p /tmp/m3
                          mount --rbind /root /tmp/m3

      I won't draw the tree..but it has 24 vfsmounts


   at step i the number of vfsmounts is V[i] = i*V[i-1].
   This is an exponential function. And this tree has way more
   mounts than what we really needed in the first place.

   One could use a series of umount at each step to prune
   out the unneeded mounts. But there is a better solution.
   Unclonable mounts come in handy here.

   step 1:
      let's say the root tree has just two directories with
      one vfsmount::

                                    root
                                   /    \
                                  tmp    usr

         How do we set up the same tree at multiple locations under
         /root/tmp

   step 2:
      ::


                        mount --bind /root/tmp /root/tmp

                        mount --make-rshared /root
                        mount --make-unbindable /root/tmp

                        mkdir -p /tmp/m1

                        mount --rbind /root /tmp/m1

      the new tree now looks like this::

                                    root
                                   /    \
                                 tmp    usr
                                /
                               m1
                              /  \
                             tmp  usr

   step 3:
      ::

                            mkdir -p /tmp/m2
                            mount --rbind /root /tmp/m2

      the new tree now looks like this::

                                    root
                                   /    \
                                 tmp    usr
                                /   \
                               m1     m2
                              /  \     / \
                             tmp  usr tmp usr

   step 4:
      ::

                            mkdir -p /tmp/m3
                            mount --rbind /root /tmp/m3

      the new tree now looks like this::

                                          root
                                      /           \
                                     tmp           usr
                                 /    \    \
                               m1     m2     m3
                              /  \     / \    /  \
                             tmp  usr tmp usr tmp usr

```
### 8) 瀹炵幇锛圛mplementation锛?


A) 鏁版嵁缁撴瀯锛圖atastructure锛?

   涓?struct vfsmount 寮曞叆浜嗗嚑鏉℃柊瀛楁锛?

   ->mnt_share
           灏嗘墍鏈変粠杩欎釜 vfsmount 鍙戦€?鎺ユ敹浼犳挱浜嬩欢鐨勬寕杞介摼鎺ュ湪涓€璧枫€?

   ->mnt_slave_list
           閾炬帴杩欎釜 vfsmount 浼犳挱鍒扮殑鎵€鏈夋寕杞姐€?

   ->mnt_slave
           灏嗚繖涓?vfsmount 鐨勪富锛坢aster锛塿fsmount 浼犳挱鍒扮殑鎵€鏈変粠灞為摼鎺ュ湪涓€璧枫€?

   ->mnt_master
           鎸囧悜杩欎釜 vfsmount 浠庝腑鎺ユ敹浼犳挱鐨勪富 vfsmount銆?

   ->mnt_flags
           澧炲姞浜嗕袱涓爣蹇椾綅锛岀敤浜庢寚绀?vfsmount 鐨勪紶鎾姸鎬併€侻NT_SHARE 琛ㄧず璇?vfsmount 鏄竴涓叡浜?vfsmount銆侻NT_UNCLONABLE 琛ㄧず璇?vfsmount 涓嶈兘琚鍒躲€?

   涓€涓绛夌粍涓殑鎵€鏈夊叡浜?vfsmount 閫氳繃 ->mnt_share 褰㈡垚涓€涓惊鐜摼琛ㄣ€?

   鎵€鏈夊叿鏈夌浉鍚?->mnt_master 鐨?vfsmount 褰㈡垚涓€涓惊鐜摼琛紝璇ラ摼琛ㄩ敋瀹氬湪 ->mnt_master->mnt_slave_list 涓紝骞堕€氳繃 ->mnt_slave 涓茶仈銆?

   ->mnt_master 鍙互鎸囧悜涓诲绛夌粍浠绘剰锛堜笖鍙兘涓嶅悓锛夌殑鎴愬憳銆傝鎵惧埌涓€涓绛夌粍鐨勬墍鏈夌洿鎺ヤ粠灞烇紝闇€瑕侀亶鍘嗗叾鎵€鏈夋垚鍛樼殑 _鎵€鏈塤 ->mnt_slave_list銆備粠姒傚康涓婅瀹冩槸涓€涓崟涓€鐨勯泦鍚堚€斺€斿垎甯冨埌鍚勪釜閾捐〃涓婂苟涓嶅奖鍝嶄紶鎾紝涔熶笉褰卞搷鎿嶄綔瀵逛紶鎾爲鐨勪慨鏀规柟寮忋€?

   涓€涓绛夌粍涓殑鎵€鏈?vfsmount 鎷ユ湁鐩稿悓鐨?->mnt_master銆傚鏋滃畠闈?NULL锛屽畠浠氨褰㈡垚涓€涓繛缁殑锛堟湁搴忕殑锛変粠灞為摼琛ㄦ銆?

   涓€涓ず渚嬩紶鎾爲濡備笅鍥炬墍绀恒€?


```
      Though it looks like a forest, if we consider all the shared
      mounts as a conceptual entity called 'pnode', it becomes a tree.

   ::


                        A <--> B <--> C <---> D
                       /|\            /|      |\
                      / F G          J K      H I
                     /
                    E<-->K
                        /|\
                       M L N

   In the above figure  A,B,C and D all are shared and propagate to each
   other.   'A' has got 3 slave mounts 'E' 'F' and 'G' 'C' has got 2 slave
   mounts 'J' and 'K'  and  'D' has got two slave mounts 'H' and 'I'.
   'E' is also shared with 'K' and they propagate to each other.  And
   'K' has 3 slaves 'M', 'L' and 'N'

   A's ->mnt_share links with the ->mnt_share of 'B' 'C' and 'D'

   A's ->mnt_slave_list links with ->mnt_slave of 'E', 'K', 'F' and 'G'

   E's ->mnt_share links with ->mnt_share of K

   'E', 'K', 'F', 'G' have their ->mnt_master point to struct vfsmount of 'A'

   'M', 'L', 'N' have their ->mnt_master point to struct vfsmount of 'K'

   K's ->mnt_slave_list links with ->mnt_slave of 'M', 'L' and 'N'

   C's ->mnt_slave_list links with ->mnt_slave of 'J' and 'K'

   J and K's ->mnt_master points to struct vfsmount of C

   and finally D's ->mnt_slave_list links with ->mnt_slave of 'H' and 'I'

   'H' and 'I' have their ->mnt_master pointing to struct vfsmount of 'D'.


   NOTE: The propagation tree is orthogonal to the mount tree.

```
B) 鍔犻攣锛圠ocking锛夛細

   ->mnt_share銆?>mnt_slave銆?>mnt_slave_list銆?>mnt_master 鐢?namespace_sem 淇濇姢锛堜慨鏀规椂鐙崰锛岃鍙栨椂鍏变韩锛夈€?

   閫氬父鎴戜滑閫氳繃 vfsmount_lock 鏉ヤ覆琛屽寲 ->mnt_flags 鐨勪慨鏀广€傛湁涓や釜渚嬪锛歞o_add_mount() 鍜?clone_mnt()銆傚墠鑰呬慨鏀逛竴涓皻鏈湪浠讳綍鍏变韩鏁版嵁缁撴瀯涓彲瑙佺殑 vfsmount銆傚悗鑰呮寔鏈?namespace_sem锛屼笖瀵?vfsmount 鐨勫敮涓€寮曠敤閮戒綅浜庝笉鎸佹湁 namespace_sem 灏辨棤娉曢亶鍘嗙殑閾捐〃涓€?

C) 绠楁硶锛圓lgorithm锛夛細

   瀹炵幇鐨勬牳蹇冨湪浜?rbind/move 鎿嶄綔銆?

   鎬讳綋绠楁硶灏嗚鎿嶄綔鍒嗚В涓?3 涓樁娈碉細锛堝弬瑙?attach_recursive_mnt() 鍜?propagate_mnt()锛?

   1. 鍑嗗闃舵锛圥repare phase锛夈€?

      瀵逛簬婧愭爲涓殑姣忎釜鎸傝浇锛?

      a) 鍒涘缓鎵€闇€鏁伴噺鐨勬寕杞芥爲锛屼互闄勫姞鍒颁粠鐩爣鎸傝浇鎺ユ敹浼犳挱鐨勬墍鏈夋寕杞戒笂銆?
      b) 涓嶈灏嗕换浣曟爲闄勫姞鍒板叾鐩爣涓娿€備絾瑕佽褰曞叾 ->mnt_parent 鍜?->mnt_mountpoint銆?
      c) 灏嗘墍鏈夋柊鎸傝浇閾炬帴璧锋潵锛屽舰鎴愪竴涓笌鐩爣鎸傝浇鐨勪紶鎾爲瀹屽叏鐩稿悓鐨勪紶鎾爲銆?

      濡傛灉姝ら樁娈垫垚鍔燂紝搴斿綋鍒涘缓 'n' 涓柊鐨勪紶鎾爲锛屽叾涓?'n' 鏄簮鏍戜腑鎸傝浇鐨勬暟閲忋€傝繘鍏ユ彁浜ら樁娈点€?

      鍚屾椂搴斿綋鍒涘缓 'm' 涓柊鐨勬寕杞芥爲锛屽叾涓?'m' 鏄洰鏍囨寕杞戒紶鎾埌鐨勬寕杞芥暟閲忋€?

      濡傛灉浠讳綍鍐呭瓨鍒嗛厤澶辫触锛岃繘鍏ヤ腑姝㈤樁娈点€?

   2. 鎻愪氦闃舵锛圕ommit phase锛夈€?

      灏嗘瘡涓寕杞芥爲闄勫姞鍒板叾瀵瑰簲鐨勭洰鏍囨寕杞戒笂銆?

   3. 涓闃舵锛圓bort phase锛夈€?

      鍒犻櫎鎵€鏈夋柊鍒涘缓鐨勬爲銆?


```
      all the propagation related functionality resides in the file pnode.c


```
------------------------------------------------------------------------

version 0.1  (created the initial document, Ram Pai linuxram@us.ibm.com)

version 0.2  (Incorporated comments from Al Viro)

