import type { Meta, StoryObj } from '@storybook/vue3';

import FormMessage from '../components/ui/form/FormMessage.vue';

const meta = {
  title: 'FormMessage',
  component: FormMessage,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof FormMessage>;

export default meta;
type Story = StoryObj<typeof FormMessage>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};