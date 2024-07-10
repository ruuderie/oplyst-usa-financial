import type { Meta, StoryObj } from '@storybook/vue3';

import FormDescription from '../components/ui/form/FormDescription.vue';

const meta = {
  title: 'FormDescription',
  component: FormDescription,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof FormDescription>;

export default meta;
type Story = StoryObj<typeof FormDescription>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};