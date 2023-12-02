import { Container } from "@react-email/container";
import { Heading } from "@react-email/heading";
import { Text } from "@react-email/text";
import { Button } from "@react-email/button";
import { Section } from "@react-email/section";
import { Tailwind } from "@react-email/tailwind";
function ResetPasswordEmail() {
    return <Tailwind componentId="1">

      <Container componentId="2">

        <Section componentId="3">

          <Heading as="h1" className="text-3xl font-bold mb-4" componentId="4">

            Reset Your Password

          </Heading>

          <p>1) what</p>

          <Text className="mb-4" componentId="5">

            We received a request to reset the password for your account. If you

            did not make this request, please ignore this email.

          </Text>

          <Text className="mb-8" componentId="6">

            To reset your password, click the button below:

          </Text>

          <Button href="\#" className="bg-blue-600 text-white font-bold py-2 px-4 rounded" componentId="7">

            Reset Password

          </Button>

        </Section>

      </Container>

    </Tailwind>;
}
export default ResetPasswordEmail;
