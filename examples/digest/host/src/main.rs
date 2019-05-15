use optee_teec::{
    Context, Operation, ParamNone, ParamTmpRef, ParamType, ParamValue, Session, Uuid,
};
use optee_teec::{Error, ErrorKind};
use proto::{Command, UUID};
use std::env;

fn update(session: &mut Session, src: &[u8]) -> optee_teec::Result<()> {
    let p0 = ParamTmpRef::new_input(src);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);

    session.invoke_command(Command::Update as u32, &mut operation)?;
    Ok(())
}

fn do_final(session: &mut Session, src: &[u8], res: &mut [u8]) -> optee_teec::Result<usize> {
    let p0 = ParamTmpRef::new_input(src);
    let p1 = ParamTmpRef::new_output(res);
    let p2 = ParamValue::new(0, 0, ParamType::ValueOutput);
    let mut operation = Operation::new(0, p0, p1, p2, ParamNone);

    session.invoke_command(Command::DoFinal as u32, &mut operation)?;

    Ok(operation.parameters().2.a() as usize)
}

fn main() -> optee_teec::Result<()> {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    if args_len < 2 {
        println!("Do not receive any message for digest.");
        println!("Correct usage: passed more than 1 argument as <part_of_message>");
        return Err(Error::new(ErrorKind::BadParameters));
    }

    let mut ctx = Context::new()?;
    let uuid = Uuid::parse_str(UUID).unwrap();

    let mut hash: [u8; 32] = [0u8; 32];
    let mut session = ctx.open_session(uuid)?;

    for i in 1..args_len - 1 {
        update(&mut session, args[i].as_bytes())?;
    }

    let hash_length = do_final(&mut session, args[args_len - 1].as_bytes(), &mut hash).unwrap();
    let mut res = hash.to_vec();
    res.truncate(hash_length as usize);

    println!("Get message hash as: {:?}.", res);

    println!("Success");
    Ok(())
}
