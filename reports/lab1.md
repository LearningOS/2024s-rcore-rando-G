为taskcontrolblock维护两个变量
    
///任务的开始时间
pub start_time:usize,
    
///syscall数组
pub syscall_times:[u32;MAX_SYSCALL_NUM]

然后在对sys_task_info的参数_ti中，通过新增接口的方式来获取status和syscalltimes和time。

我们可以通过在TaskManger的全局实现中实现这几个接口，而这需要我们新增几个方法去维护。